use anyhow::Result;

use super::stdio_server::run_mcp_stdio;
use super::streamable_http::run_mcp_http;
use super::transport::TransportType;

/// Default address the HTTP transport binds to.
pub const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1";

/// Default port for the HTTP transport.
pub const DEFAULT_PORT: u16 = 8000;

/// Default route path for the MCP endpoint.
pub const DEFAULT_ROUTE_PATH: &str = "/rudof";

/// Browser origins accepted when [`McpConfig::allowed_origins`] is empty.
///
/// Loopback only, which is what a locally running server should answer. rmcp
/// disables `Origin` validation entirely when its allowlist is empty, so this
/// list has to be non-empty for the check to run at all.
pub const DEFAULT_ALLOWED_ORIGINS: &[&str] = &[
    "http://localhost:*",
    "https://localhost:*",
    "http://127.0.0.1:*",
    "https://127.0.0.1:*",
    "http://[::1]:*",
    "https://[::1]:*",
];

#[derive(Clone, Debug)]
/// Configuration for the MCP server.
pub struct McpConfig {
    /// Transport type to use (Stdio or StreamableHTTP)
    pub transport: TransportType,
    /// Address to bind the HTTP server to (ignored for Stdio). Example: "127.0.0.1"
    pub bind_address: String,
    /// Port for HTTP transport (ignored for Stdio)
    pub port: u16,
    /// Endpoint route path for HTTP (ignored for Stdio). Example: "/mcp"
    pub route_path: String,
    /// Browser origins accepted by `Origin` validation (ignored for Stdio).
    /// Entries are matched per RFC 6454 on (scheme, host, port), and should
    /// carry an explicit port or `:*`. Empty means [`DEFAULT_ALLOWED_ORIGINS`].
    pub allowed_origins: Vec<String>,
    /// Extra `Host` authorities accepted on top of rmcp's loopback default
    /// (ignored for Stdio). Needed when clients reach the server by a name or
    /// address other than localhost. Example: `["example.com", "10.0.0.5"]`.
    pub allowed_hosts: Vec<String>,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            transport: TransportType::Stdio,
            bind_address: DEFAULT_BIND_ADDRESS.to_string(),
            port: DEFAULT_PORT,
            route_path: DEFAULT_ROUTE_PATH.to_string(),
            allowed_origins: Vec::new(),
            allowed_hosts: Vec::new(),
        }
    }
}

/// Entry point for running the MCP server with the specified transport.
/// This function routes to the appropriate transport implementation:
/// - TransportType::Stdio: Uses stdin/stdout for communication (for CLI tools, IDE extensions, etc.)
/// - TransportType::StreamableHTTP: Uses HTTP with SSE for communication (for web clients, network access, etc.)
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
        TransportType::StreamableHTTP => run_mcp_http(&config).await,
    }
}
