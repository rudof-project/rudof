use clap::Args;
use rudof_mcp::server::{DEFAULT_BIND_ADDRESS, DEFAULT_PORT, DEFAULT_ROUTE_PATH, TransportType};

/// Arguments for the `mcp` command
#[derive(Debug, Clone, Args)]
pub struct McpArgs {
    #[arg(
        short = 't',
        long = "transport",
        value_name = "TRANSPORT",
        ignore_case = true,
        help = "Transport type: stdio (for CLI/IDE) or streamable-http (for web clients)",
        default_value_t = TransportType::Stdio
    )]
    pub transport: TransportType,

    #[arg(
        short = 'b',
        long = "bind",
        value_name = "ADDRESS",
        help = "Bind address for the streamable-http transport. Examples: 127.0.0.1 (localhost IPv4), \
              0.0.0.0 (all IPv4 interfaces), ::1 (localhost IPv6), :: (all IPv6 interfaces). \
              Default: 127.0.0.1 for security",
        default_value = DEFAULT_BIND_ADDRESS
    )]
    pub bind_address: String,

    #[arg(
        short = 'p',
        long = "port",
        value_name = "PORT",
        help = "Port number (only used with the streamable-http transport)",
        default_value_t = DEFAULT_PORT
    )]
    pub port: u16,

    #[arg(
        short = 'r',
        long = "route",
        value_name = "PATH",
        help = "Route path for the MCP endpoint (only used with the streamable-http transport)",
        default_value = DEFAULT_ROUTE_PATH
    )]
    pub route_path: String,

    #[arg(
        short = 'o',
        long = "allowed-origin",
        value_name = "ORIGIN",
        help = "Browser origin allowed to call the server (only used with the streamable-http transport). \
              Can be comma-separated or specified multiple times. Each entry needs a scheme and should \
              carry an explicit port or ':*'. Examples: http://localhost:*, https://app.example.com:443. \
              If not specified, only loopback origins are allowed",
        num_args = 0..,
        value_delimiter = ','
    )]
    pub allowed_origins: Vec<String>,

    #[arg(
        short = 'H',
        long = "allowed-host",
        value_name = "HOST",
        help = "Host authority clients may use to reach the server, on top of loopback (only used with \
              the streamable-http transport). Can be comma-separated or specified multiple times. \
              Required when binding to all interfaces. Examples: example.com, example.com:8080, 10.0.0.5",
        num_args = 0..,
        value_delimiter = ','
    )]
    pub allowed_hosts: Vec<String>,
}
