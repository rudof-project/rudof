use crate::cli::parser::DCTapArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `dctap` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute DCTap server logic.
pub struct DctapCommand {
    /// Arguments specific to DCTap server.
    args: DCTapArgs,
}

impl DctapCommand {
    pub fn new(args: DCTapArgs) -> Self {
        Self { args }
    }
}

impl Command for DctapCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "dctap"
    }

    /// Executes the DCTap server logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        ctx.rudof.load_dctap(&self.args.file, Some(&self.args.format.into()));

        ctx.rudof.serialize_dctap(Some(&self.args.result_format.into()), &mut ctx.writer);

        Ok(())
    }
}
