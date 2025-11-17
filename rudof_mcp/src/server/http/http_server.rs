use std::sync::Arc;

use anyhow::Result;
use axum::{
    Router,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{any_service, delete},
};
use rmcp::transport::streamable_http_server::{
    SessionManager, StreamableHttpService, session::local::LocalSessionManager,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use super::auth::{AuthConfig, authorization_guard, protected_resource_metadata_handler};
use super::config::AS_URL;
use super::middleware::with_guards;
use crate::rudof_mcp_service::RudofMcpService;

/// Run MCP server using HTTP with Server-Sent Events (SSE) transport.
/// This transport is ideal for:
/// - Web-based MCP clients
/// - Remote connections over HTTP/HTTPS
/// - Multiple concurrent client connections
/// - Network-based integrations
///
/// # Security Features
/// - OAuth2/OIDC authentication with JWT validation
/// - TLS/HTTPS support (configure via reverse proxy)
/// - JWT signature verification using JWKS
/// - Audience and issuer validation (RFC 8707)
/// - DNS rebinding protection (origin validation)
/// - Localhost-only binding by default
///
/// # Protocol
/// - MCP 2025-06-18 Streamable HTTP transport
/// - JSON-RPC over HTTP POST
/// - Server-Sent Events for real-time notifications
/// - Session management with explicit termination support
pub async fn run_mcp_http(port: u16, route_path: &str) -> Result<()> {
    let bind_addr = format!("localhost:{}", port);
    let canonical_uri = format!("http://localhost:{}{}", port, route_path);

    let session_manager = Arc::new(LocalSessionManager::default());

    let mcp_service_factory = move || Ok(RudofMcpService::new());

    let rmcp_service = StreamableHttpService::new(
        mcp_service_factory,
        session_manager.clone(),
        Default::default(),
    );

    let oauth2_auth_cfg = Arc::new(
        AuthConfig::new(canonical_uri.clone(), AS_URL.to_string(), true)
            .with_cache_ttl(std::time::Duration::from_secs(3600)),
    );

    // Build routes
    let router = Router::new()
        // MCP session endpoint with DELETE handler
        .route(
            route_path,
            delete({
                let sm = session_manager.clone();
                move |headers| handle_delete_session(headers, sm.clone())
            })
            .fallback_service(any_service(rmcp_service)),
        )
        // OAuth2 Protected Resource Metadata (RFC 9728)
        .route(
            "/.well-known/oauth-protected-resource",
            axum::routing::get({
                let uri = canonical_uri.clone();
                move || protected_resource_metadata_handler(uri.clone())
            }),
        );

    // Configure CORS layer
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any) // Allow any origin for development
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::HeaderName::from_static("mcp-protocol-version"),
            axum::http::HeaderName::from_static("mcp-session-id"),
        ])
        .expose_headers([axum::http::HeaderName::from_static("mcp-session-id")]);

    // Apply tracing and CORS layers (order matters: CORS first, then tracing)
    let router = router.layer(TraceLayer::new_for_http()).layer(cors);

    // Apply protocol and origin guards (before authorization)
    let guarded_router = with_guards(router);

    // Apply authorization middleware (after guards)
    let guarded_router = guarded_router.layer(axum::middleware::from_fn_with_state(
        oauth2_auth_cfg.clone(),
        authorization_guard,
    ));

    let listener = std::net::TcpListener::bind(&bind_addr)?;
    let server = axum_server::Server::from_tcp(listener).serve(guarded_router.into_make_service());

    tracing::info!("MCP HTTP server listening on {}", canonical_uri);
    tracing::info!("Transport: HTTP with Server-Sent Events (SSE)");
    tracing::info!("Authorization server: {}", oauth2_auth_cfg.issuer);
    tracing::info!("Session management: Enabled with explicit termination support");

    // Graceful shutdown handling
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutdown signal received, stopping HTTP server...");
        }
    }

    Ok(())
}

/// Handler for explicit session termination (HTTP DELETE).
///
/// According to the MCP specification:
/// - Clients may send a DELETE with the `Mcp-Session-Id` header to explicitly end a session
/// - The server MUST respond:
///   - `204 No Content` if the session was terminated successfully
///   - `404 Not Found` if the session was not found or already expired
///   - `400 Bad Request` if the header is missing or invalid
async fn handle_delete_session(
    headers: HeaderMap,
    session_manager: Arc<LocalSessionManager>,
) -> impl IntoResponse {
    match headers.get("Mcp-Session-Id").and_then(|v| v.to_str().ok()) {
        Some(id) => {
            let id_arc = Arc::from(id.to_string());
            match session_manager.close_session(&id_arc).await {
                Ok(()) => {
                    tracing::info!(session_id = %id, "Session terminated successfully");
                    (StatusCode::NO_CONTENT, "").into_response()
                }
                Err(e) => {
                    tracing::warn!(session_id = %id, error = %e, "Session not found or already expired");
                    (
                        StatusCode::NOT_FOUND,
                        "Session not found or already expired",
                    )
                        .into_response()
                }
            }
        }
        None => {
            tracing::warn!("Missing Mcp-Session-Id header in DELETE request");
            (StatusCode::BAD_REQUEST, "Missing Mcp-Session-Id header").into_response()
        }
    }
}
