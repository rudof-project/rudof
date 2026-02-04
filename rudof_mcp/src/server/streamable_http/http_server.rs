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

use super::middleware::with_guards;
use crate::service::RudofMcpService;

/// Run MCP server using HTTP with Server-Sent Events (SSE) transport.
/// This transport is ideal for:
/// - Web-based MCP clients
/// - Remote connections over HTTP/HTTPS
/// - Multiple concurrent client connections
/// - Network-based integrations
///
/// # Security Features (per MCP spec 2025-11-25)
/// - Origin header validation: If present and invalid, returns HTTP 403 Forbidden
/// - MCP protocol version validation: Invalid/unsupported returns HTTP 400 Bad Request
/// - Localhost-only binding by default (127.0.0.1, not 0.0.0.0)
/// - Session management with explicit termination support
///
/// # Protocol (MCP 2025-11-25)
/// - JSON-RPC over HTTP POST (single request/notification/response per POST)
/// - Server-Sent Events for real-time streaming
/// - Session management with MCP-Session-Id header
/// - Supports backwards compatibility with 2025-06-18 and 2025-03-26
///
/// # Note on CORS
/// CORS is not configured here. If you need CORS for browser-based clients,
/// configure it in a reverse proxy (Nginx, Caddy, etc.) for better flexibility.
pub async fn run_mcp_http(port: u16, route_path: &str) -> Result<()> {
    // Bind to 127.0.0.1 explicitly for security per MCP spec
    let bind_addr = format!("127.0.0.1:{}", port);
    let canonical_uri = format!("http://127.0.0.1:{}{}", port, route_path);

    let session_manager = Arc::new(LocalSessionManager::default());

    let mcp_service_factory = move || Ok(RudofMcpService::new());

    let rmcp_service = StreamableHttpService::new(
        mcp_service_factory,
        session_manager.clone(),
        Default::default(),
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
        );

    // Apply protocol and origin guards
    let guarded_router = with_guards(router);

    let listener = std::net::TcpListener::bind(&bind_addr)?;
    let server = axum_server::Server::from_tcp(listener).serve(guarded_router.into_make_service());

    tracing::info!("MCP HTTP server listening on {}", canonical_uri);

    // Graceful shutdown handling
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::debug!("Shutdown signal received, stopping HTTP server...");
        }
    }

    Ok(())
}

/// Handler for explicit session termination (HTTP DELETE).
///
/// According to the MCP specification (2025-11-25):
/// - Clients that no longer need a session SHOULD send HTTP DELETE with MCP-Session-Id header
/// - The server MAY respond with HTTP 405 Method Not Allowed if it doesn't allow session termination
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
                    tracing::error!(session_id = %id, error = %e, "Session not found or already expired");
                    (
                        StatusCode::NOT_FOUND,
                        "Session not found or already expired",
                    )
                        .into_response()
                }
            }
        }
        None => {
            tracing::error!("Missing Mcp-Session-Id header in DELETE request");
            (StatusCode::BAD_REQUEST, "Missing Mcp-Session-Id header").into_response()
        }
    }
}
