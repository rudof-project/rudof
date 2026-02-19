use crate::cli::parser::ShapemapArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `shapemap` command.
/// 
/// This struct holds the specific arguments parsed by `clap` and 
/// implements the [Command] trait to execute Shapemap logic.
pub struct ShapemapCommand {
    /// Arguments specific to shapemap.
    args: ShapemapArgs,
}

impl ShapemapCommand {
    pub fn new(args: ShapemapArgs) -> Self {
        Self { args }
    }
}

impl Command for ShapemapCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shapemap"
    }

    /// Executes the shapemap logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Shapemap command executed");
        Ok(())
    }
}