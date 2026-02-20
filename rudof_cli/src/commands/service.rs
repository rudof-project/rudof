use crate::cli::parser::ServiceArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `service` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Service command logic.
pub struct ServiceCommand {
    /// Arguments specific to Service command.
    args: ServiceArgs,
}

impl ServiceCommand {
    pub fn new(args: ServiceArgs) -> Self {
        Self { args }
    }
}

impl Command for ServiceCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "service"
    }

    /// Executes the Service command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        println!("Service command executed");
        Ok(())
    }
}
