use std::sync::Arc;

use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{delete, any_service},
    Router,
};
use anyhow::Result;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService,
    session::local::LocalSessionManager,
    SessionManager,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

use crate::{
    middleware::with_guards,
    rudof_mcp_service::RudofMcpService,
    auth::{
        AuthConfig,
        protected_resource_metadata_handler,
        authorization_guard,
    },
    config::{CANONICAL_URI, AS_URL, ROUTE_PATH, BIND_ADDR}
};

/// Entry point for running the MCP Streamable HTTP server.
/// 
/// This function sets up the MCP server according to the MCP 2025-06-18 specification:
/// - Initializes the `StreamableHttpService` with a `LocalSessionManager`
/// - Builds and applies OAuth2 authentication with proper token validation
/// - Exposes OAuth2 discovery endpoints (RFC 9728, RFC 8414)
/// - Applies protocol and origin guards before authorization middleware
/// - Binds the Axum server with graceful shutdown handling
///
/// # Security Features
/// - JWT signature verification using JWKS from authorization server
/// - Audience validation (RFC 8707) - tokens must target this specific server
/// - Issuer validation - tokens must come from configured authorization server
/// - Expiration and not-before time validation with clock skew tolerance
/// - JWKS caching to reduce network overhead
/// - Proper WWW-Authenticate headers on 401 responses (RFC 9728)
#[tokio::main]
pub async fn run_mcp() -> Result<()> {
    let session_manager = Arc::new(LocalSessionManager::default());
    let mcp_service_factory = || Ok(RudofMcpService::new());
    let rmcp_service = StreamableHttpService::new(
        mcp_service_factory,
        session_manager.clone(),
        Default::default(),
    );

    let oauth2_auth_cfg = Arc::new(
        AuthConfig::new(
            CANONICAL_URI.to_string(),
            AS_URL.to_string(),
            true,
        )
        .with_cache_ttl(std::time::Duration::from_secs(3600))
    );

    // Build routes
    let router = Router::new()
        // MCP session endpoint with DELETE handler
        .route(
            ROUTE_PATH,
            delete({
                let sm = session_manager.clone();
                move |headers| handle_delete_session(headers, sm.clone())
            })
            .fallback_service(any_service(rmcp_service)),
        )
        // OAuth2 Protected Resource Metadata (RFC 9728)
        // Base endpoint - returns metadata for default resource
        .route(
            "/.well-known/oauth-protected-resource",
            axum::routing::get({
                move || protected_resource_metadata_handler(CANONICAL_URI.to_string())
            }),
        );

    // Apply CORS
    let router = router.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
            .allow_headers(Any),
    );

    // Apply tracing
    let router = router.layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // Apply protocol and origin guards (before authorization)
    let guarded_router = with_guards(router);

    // Apply authorization middleware (after guards)
    let guarded_router = guarded_router.layer(
        axum::middleware::from_fn_with_state(
            oauth2_auth_cfg.clone(),
            authorization_guard,
        )
    );

    let listener = std::net::TcpListener::bind(BIND_ADDR)?;
    let server = axum_server::Server::from_tcp(listener)
        .serve(guarded_router.into_make_service());

    tracing::info!("MCP Server listening on {}", CANONICAL_URI);
    tracing::info!("Authorization server: {}", oauth2_auth_cfg.issuer);

    // Graceful shutdown handling
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutdown signal received, stopping server...");
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
                    (StatusCode::NOT_FOUND, "Session not found or already expired").into_response()
                }
            }
        }
        None => {
            tracing::warn!("Missing Mcp-Session-Id header in DELETE request");
            (StatusCode::BAD_REQUEST, "Missing Mcp-Session-Id header").into_response()
        }
    }
}
