use anyhow::Result;

use super::streamable_http::run_mcp_http;
use super::stdio_server::run_mcp_stdio;
use super::transport::TransportType;

/// Entry point for running the MCP server with the specified transport.
/// This function routes to the appropriate transport implementation:
/// - TransportType::Stdio: Uses stdin/stdout for communication (for CLI tools, IDE extensions, etc.)
/// - TransportType::StreamableHTTP: Uses HTTP with SSE for communication (for web clients, network access, etc.)
///
/// # Note
/// This function creates its own Tokio runtime. If you're already in an async context,
/// use `run_mcp_async` instead.
#[tokio::main]
pub async fn run_mcp(transport: TransportType, port: u16, route_path: &str) -> Result<()> {
    run_mcp_async(transport, port, route_path).await
}

/// Async version of `run_mcp` for use within an existing Tokio runtime.
pub async fn run_mcp_async(transport: TransportType, port: u16, route_path: &str) -> Result<()> {
    match transport {
        TransportType::Stdio => run_mcp_stdio().await,
        TransportType::StreamableHTTP => run_mcp_http(port, route_path).await,
    }
}
