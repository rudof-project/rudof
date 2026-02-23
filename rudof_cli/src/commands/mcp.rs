use crate::cli::parser::McpArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_mcp::server::{run_mcp, McpConfig};

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
            bind_address: Some(self.args.bind_address.to_string()),
            port: Some(self.args.port),
            route_path: Some(self.args.route_path.to_string()),
            allowed_networks: Some(self.args.allowed_networks.clone()),
        };

        run_mcp(mcp_config)?;

        Ok(())
    }
}
