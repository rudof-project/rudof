use crate::cli::parser::ConvertArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `convert` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Convert command logic.
pub struct ConvertCommand {
    /// Arguments specific to Convert command.
    args: ConvertArgs,
}

impl ConvertCommand {
    pub fn new(args: ConvertArgs) -> Self {
        Self { args }
    }
}

impl Command for ConvertCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "convert"
    }

    /// Executes the Convert command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Convert command executed");
        Ok(())
    }
}
