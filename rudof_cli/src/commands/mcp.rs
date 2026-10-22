use crate::cli::parser::McpArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_mcp::server::{McpConfig, run_mcp};

/// Implementation of the `mcp` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute MCP server logic.
pub struct McpCommand {
    /// Arguments specific to MCP server.
    args: McpArgs,
}

impl McpCommand {
    pub fn new(args: McpArgs) -> Self {
        Self { args }
    }
}

impl Command for McpCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "mcp"
    }

    /// Executes the MCP server logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        let mcp_config = McpConfig {
            transport: self.args.transport,
            bind_address: self.args.bind_address.clone(),
            port: self.args.port,
            route_path: self.args.route_path.clone(),
            allowed_origins: self.args.allowed_origins.clone(),
            allowed_hosts: self.args.allowed_hosts.clone(),
        };

        run_mcp(mcp_config)?;

        Ok(())
    }
}
