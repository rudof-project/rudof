use crate::cli::parser::QueryArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `query` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Query command logic.
pub struct QueryCommand {
    /// Arguments specific to Query command.
    args: QueryArgs,
}

impl QueryCommand {
    pub fn new(args: QueryArgs) -> Self {
        Self { args }
    }
}

impl Command for QueryCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "query"
    }

    /// Executes the Query command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Query command executed");
        Ok(())
    }
}
