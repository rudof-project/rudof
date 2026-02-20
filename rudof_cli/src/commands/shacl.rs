use crate::cli::parser::ShaclArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `shacl` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Shacl command logic.
pub struct ShaclCommand {
    /// Arguments specific to Shacl command.
    args: ShaclArgs,
}

impl ShaclCommand {
    pub fn new(args: ShaclArgs) -> Self {
        Self { args }
    }
}

impl Command for ShaclCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shacl"
    }

    /// Executes the Shacl command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Shacl command executed");
        Ok(())
    }
}
