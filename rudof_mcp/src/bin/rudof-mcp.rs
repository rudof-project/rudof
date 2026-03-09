//! Standalone binary for Rudof MCP Server
//! 
//! This is a dedicated binary that runs the Rudof MCP server directly.
//! It provides a simpler interface compared to the full rudof CLI tool.
//! 
//! Usage:
//!   rudof-mcp                              # Run with default stdio transport
//!   rudof-mcp --transport streamable-http  # Run with HTTP transport
//!   rudof-mcp --help                       # Show all options

use anyhow::Result;
use clap::Parser;
use rudof_mcp::server::{McpConfig, TransportType, run_mcp};

#[derive(Parser, Debug)]
#[command(
    name = "rudof-mcp",
    about = "Rudof MCP Server - RDF validation and querying via Model Context Protocol",
    version,
    long_about = "A Model Context Protocol (MCP) server that provides RDF data validation, \
                  SPARQL querying, and semantic data transformation capabilities to AI \
                  assistants and MCP-compatible clients."
)]
struct Args {
    /// Transport type: stdio (for CLI/IDE) or streamable-http (for web clients)
    #[arg(
        short = 't',
        long = "transport",
        value_name = "TRANSPORT",
        ignore_case = true,
        help = "Transport type: stdio (for CLI/IDE) or streamable-http (for web clients)",
        default_value_t = TransportType::Stdio
    )]
    transport: TransportType,

    /// Bind address for HTTP transport (ignored for stdio)
    #[arg(
        short = 'b',
        long = "bind",
        value_name = "ADDRESS",
        help = "Bind address for HTTP transport. Examples: 127.0.0.1 (localhost IPv4), \
                0.0.0.0 (all IPv4 interfaces), ::1 (localhost IPv6), :: (all IPv6 interfaces). \
                Default: 127.0.0.1 for security",
        default_value = "127.0.0.1"
    )]
    bind_address: String,

    /// Port number for HTTP transport (ignored for stdio)
    #[arg(
        short = 'p',
        long = "port",
        value_name = "PORT",
        help = "Port number for HTTP transport (only used with streamable-http transport)",
        default_value_t = 8000
    )]
    port: u16,

    /// Route path for HTTP transport (ignored for stdio)
    #[arg(
        short = 'r',
        long = "route",
        value_name = "PATH",
        help = "Route path for HTTP transport (only used with streamable-http transport)",
        default_value = "/rudof"
    )]
    route_path: String,

    /// Allowed IP networks in CIDR notation for HTTP transport
    #[arg(
        short = 'n',
        long = "allowed-network",
        value_name = "CIDR",
        help = "Allowed IP network in CIDR notation (only used with streamable-http transport). \
                Can be specified multiple times to allow multiple networks. \
                Examples: 127.0.0.1, 192.168.1.0/24, 10.0.0.0/8, ::1. \
                If not specified, defaults to localhost only (127.0.0.0/8 and ::1/128)",
        num_args = 0..
    )]
    allowed_networks: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Configure MCP server
    let config = McpConfig {
        transport: args.transport,
        bind_address: Some(args.bind_address),
        port: Some(args.port),
        route_path: Some(args.route_path),
        allowed_networks: Some(args.allowed_networks),
    };

    // Run the MCP server
    run_mcp(config)?;

    Ok(())
}
