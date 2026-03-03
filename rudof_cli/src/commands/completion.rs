use crate::cli::parser::{Cli, CompletionArgs};
use crate::commands::CommandContext;
use crate::commands::base::Command;
use clap::CommandFactory;

/// Implementation of the `compare` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [`Command`] trait to execute Compare command logic.
pub struct CompletionCommand {
    /// Arguments specific to Completion command.
    args: CompletionArgs,
}

impl CompletionCommand {
    pub fn new(args: CompletionArgs) -> Self {
        Self { args }
    }
}

impl Command for CompletionCommand {
    /// Executes the Completion command logic.
    fn execute(&self, ctx: &mut CommandContext) -> anyhow::Result<()> {
        self.args.shell.generate(&mut Cli::command(), ctx.writer.as_mut());
        Ok(())
    }

    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "completion"
    }
}
