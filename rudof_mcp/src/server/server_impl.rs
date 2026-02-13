use anyhow::Result;

use super::stdio_server::run_mcp_stdio;
use super::streamable_http::run_mcp_http;
use super::transport::TransportType;

#[derive(Clone, Debug)]
/// Configuration for the MCP server.
pub struct McpConfig {
    /// Transport type to use (Stdio or StreamableHTTP)
    pub transport: TransportType,
    /// Address to bind the HTTP server to (ignored for Stdio). Example: "127.0.0.1"
    pub bind_address: Option<String>,
    /// Port for HTTP transport (ignored for Stdio)
    pub port: Option<u16>,
    /// Endpoint route path for HTTP (ignored for Stdio). Example: "mcp"
    pub route_path: Option<String>,
    /// List of allowed networks/IPs for HTTP transport. Example: ["127.0.0.1", "192.168.1.0/24"]
    pub allowed_networks: Option<Vec<String>>,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            transport: TransportType::Stdio,
            bind_address: None,
            port: None,
            route_path: None,
            allowed_networks: None,
        }
    }
}

/// Entry point for running the MCP server with the specified transport.
/// This function routes to the appropriate transport implementation:
/// - TransportType::Stdio: Uses stdin/stdout for communication (for CLI tools, IDE extensions, etc.)
/// - TransportType::StreamableHTTP: Uses HTTP with SSE for communication (for web clients, network access, etc.)
///
/// # Arguments
/// * `transport` - The transport type to use
/// * `bind_address` - Address to bind the HTTP server to (ignored for Stdio)
/// Examples: "127.0.0.1" (localhost IPv4), "0.0.0.0" (all IPv4), "::1" (localhost IPv6)
/// * `port` - Port number for HTTP transport (ignored for Stdio)
/// * `route_path` - Route path for HTTP transport (ignored for Stdio)
/// * `allowed_networks` - Optional list of allowed IP addresses or CIDR ranges for HTTP transport .Examples: ["127.0.0.1",
/// "192.168.1.0/24", "10.0.0.0/8", "::1"]. If None or empty, defaults to localhost only
///
/// # Note
/// This function creates its own Tokio runtime. If you're already in an async context,
/// use `run_mcp_async` instead.
#[tokio::main]
pub async fn run_mcp(config: McpConfig) -> Result<()> {
    run_mcp_async(config).await
}

/// Async version of `run_mcp` for use within an existing Tokio runtime.
pub async fn run_mcp_async(config: McpConfig) -> Result<()> {
    match config.transport {
        TransportType::Stdio => run_mcp_stdio().await,
        TransportType::StreamableHTTP => {
            run_mcp_http(
                config.bind_address.as_ref().expect("bind_address required for HTTP"),
                config.port.expect("port required for HTTP"),
                config.route_path.as_ref().expect("route_path required for HTTP"),
                config.allowed_networks.clone(),
            )
            .await
        },
    }
}
