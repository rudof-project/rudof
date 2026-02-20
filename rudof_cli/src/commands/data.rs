use crate::cli::parser::DataArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `data` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Data command logic.
pub struct DataCommand {
    /// Arguments specific to Data command.
    args: DataArgs,
}

impl DataCommand {
    pub fn new(args: DataArgs) -> Self {
        Self { args }
    }
}

impl Command for DataCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "data"
    }

    /// Executes the Data command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Data command executed");
        Ok(())
    }
}
