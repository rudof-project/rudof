use crate::cli::parser::GenerateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `generate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Generate command logic.
pub struct GenerateCommand {
    /// Arguments specific to Generate command.
    args: GenerateArgs,
}

impl GenerateCommand {
    pub fn new(args: GenerateArgs) -> Self {
        Self { args }
    }
}

impl Command for GenerateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "generate"
    }

    /// Executes the Generate command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Generate command executed");
        Ok(())
    }
}
