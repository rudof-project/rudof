use crate::cli::parser::CompareArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `compare` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Compare command logic.
pub struct CompareCommand {
    /// Arguments specific to Compare command.
    args: CompareArgs,
}

impl CompareCommand {
    pub fn new(args: CompareArgs) -> Self {
        Self { args }
    }
}

impl Command for CompareCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "compare"
    }

    /// Executes the Compare command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Compare command executed");
        Ok(())
    }
}
