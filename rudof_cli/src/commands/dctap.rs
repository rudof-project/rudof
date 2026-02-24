use crate::cli::parser::DCTapArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::{dctap_format::DCTapFormat, DCTAPFormat};

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
        let format: DCTapFormat = (&self.args.format).into();
        let format: DCTAPFormat = format.into();
        let result_format = (&self.args.result_format).into();

        ctx.rudof.process_dctap(
            &self.args.file,
            &format,
            &result_format,
            &mut ctx.writer,
        )?;

        Ok(())
    }
}
