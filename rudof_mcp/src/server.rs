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

use crate::{config::ServerConfig, middleware::with_guards, rudof_mcp_service::RudofMcpService, 
    auth::{AuthConfig, protected_resource_metadata_handler, authorization_guard}};

/// Entry point for running the MCP Streamable HTTP server.
/// This function sets up the MCP server according to the **MCP 2025-06-18** specification:
/// - Initializes the `StreamableHttpService` with a `LocalSessionManager` for per-session state.
/// - Builds and applies authentication (`AuthConfig`) to enforce OAuth-protected resource access.
/// - Exposes the public discovery endpoint at `/.well-known/oauth-protected-resource`.
/// - Applies protocol and origin guards before authorization middleware, per MCP security section.
/// - Binds the Axum server with graceful shutdown handling.
#[tokio::main]
pub async fn run_mcp(route_name: &str, port: &str, host: &str) -> Result<()> {
    let cfg = ServerConfig {
        route_name: route_name.to_string(),
        host: host.to_string(),
        port: port.parse::<u16>()?,
    };

    let session_manager = Arc::new(LocalSessionManager::default());

    let service_factory = || -> std::io::Result<RudofMcpService> {
        Ok(RudofMcpService::new())
    };

    let rmcp_service = StreamableHttpService::new(
        service_factory,
        session_manager.clone(),
        Default::default(),
    );

    let route_path = format!("/{}", cfg.route_name);

    // --- Build auth config ---
    let auth_cfg = Arc::new(AuthConfig::new(
        cfg.canonical_uri(),                  // MCP canonical URI (audience)
        "https://auth.example.com".to_string(), // EXTERNAL AS base URL
        true,
    ));

    // --- Public discovery endpoint ---
    let resource_metadata_route = "/.well-known/oauth-protected-resource";

    let router = Router::new()
        .route(
            &route_path,
            delete({
                let sm = session_manager.clone();
                move |headers| handle_delete_session(headers, sm.clone())
            })
            .fallback_service(any_service(rmcp_service)),
        )
        .route(resource_metadata_route, axum::routing::get({
            let ac = auth_cfg.clone();
            move || protected_resource_metadata_handler(ac.clone())
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // --- Apply security layers ---
    // Apply protocol/origin guards first
    let guarded_router = with_guards(router);

    // Apply authorization middleware after guards
    let guarded_router = guarded_router.layer(axum::middleware::from_fn_with_state(
        auth_cfg.clone(),
        authorization_guard,  // Just pass the function name, don't wrap it in a closure
    ));

    // --- Start server ---
    let bind_addr = cfg.safe_bind_address();
    tracing::info!("Binding to {}", bind_addr);

    let listener = std::net::TcpListener::bind(&bind_addr)?;
    let server = axum_server::Server::from_tcp(listener)
        .serve(guarded_router.into_make_service());

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutdown requested");
        }
    }

    Ok(())
}

/// Handler for explicit session termination (HTTP DELETE).
/// According to the MCP-SPEC:
/// - Clients may send a DELETE with the `Mcp-Session-Id` header to explicitly end a session.
/// - The server MUST respond:
///   - `204 No Content` if the session was terminated successfully.
///   - `404 Not Found` if the session was not found or already expired.
///   - `400 Bad Request` if the header is missing or invalid.
/// Uses the in-memory `LocalSessionManager` to track and terminate sessions.
async fn handle_delete_session(
    headers: HeaderMap,
    session_manager: Arc<LocalSessionManager>,
) -> impl IntoResponse {
    match headers.get("Mcp-Session-Id").and_then(|v| v.to_str().ok()) {
        Some(id) => {
            let id_arc = Arc::from(id.to_string()); 
            match session_manager.close_session(&id_arc).await {
                Ok(()) => {
                    tracing::info!(session = %id, "Session terminated successfully");
                    (StatusCode::NO_CONTENT, "").into_response()
                }
                Err(e) => {
                    tracing::warn!(session = %id, error = %e, "Session not found or already expired");
                    (StatusCode::NOT_FOUND, "session not found").into_response()
                }
            }
        }
        None => {
            tracing::warn!("Missing Mcp-Session-Id header in DELETE request");
            (StatusCode::BAD_REQUEST, "Missing Mcp-Session-Id header").into_response()
        }
    }
}