use clap::Args;
use rudof_mcp::server::TransportType;

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
        help = "Bind address for HTTP transport. Examples: 127.0.0.1 (localhost IPv4), \
              0.0.0.0 (all IPv4 interfaces), ::1 (localhost IPv6), :: (all IPv6 interfaces). \
              Default: 127.0.0.1 for security",
        default_value = "127.0.0.1"
    )]
    pub bind_address: String,

    #[arg(
        short = 'p',
        long = "port",
        value_name = "PORT",
        help = "Port number for HTTP transport (only used with http-sse transport)",
        default_value_t = 8000
    )]
    pub port: u16,

    #[arg(
        short = 'r',
        long = "route",
        value_name = "PATH",
        help = "Route path for HTTP transport (only used with http-sse transport)",
        default_value = "/rudof"
    )]
    pub route_path: String,

    #[arg(
        short = 'n',
        long = "allowed-network",
        value_name = "CIDR",
        help = "Allowed IP network in CIDR notation (only used with http-sse transport). \
              Can be specified multiple times to allow multiple networks. \
              Examples: 127.0.0.1, 192.168.1.0/24, 10.0.0.0/8, ::1. \
              If not specified, defaults to localhost only (127.0.0.0/8 and ::1/128)",
        num_args = 0.. // allows multiple vales
    )]
    pub allowed_networks: Vec<String>,
}
