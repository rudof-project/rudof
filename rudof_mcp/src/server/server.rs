use anyhow::Result;

use super::http::run_mcp_http;
use super::stdio_server::run_mcp_stdio;
use super::transport::TransportType;

/// Entry point for running the MCP server with the specified transport.
/// This function routes to the appropriate transport implementation:
/// - TransportType::Stdio: Uses stdin/stdout for communication (for CLI tools, IDE extensions)
/// - TransportType::HttpSse: Uses HTTP with SSE for communication (for web clients, network access)
#[tokio::main]
pub async fn run_mcp(transport: TransportType, port: u16, route_path: &str) -> Result<()> {
    match transport {
        TransportType::Stdio => run_mcp_stdio().await,
        TransportType::HttpSse => run_mcp_http(port, route_path).await,
    }
}
