use crate::cli::parser::NodeArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `node` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Node command logic.
pub struct NodeCommand {
    /// Arguments specific to Node command.
    args: NodeArgs,
}

impl NodeCommand {
    pub fn new(args: NodeArgs) -> Self {
        Self { args }
    }
}

impl Command for NodeCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "node"
    }

    /// Executes the Node command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Node command executed");
        Ok(())
    }
}
