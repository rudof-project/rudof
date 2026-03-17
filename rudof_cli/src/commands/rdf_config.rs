use crate::cli::parser::RdfConfigArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `rdf-config` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute RdfConfig command logic.
pub struct RdfConfigCommand {
    /// Arguments specific to RdfConfig command.
    args: RdfConfigArgs,
}

impl RdfConfigCommand {
    pub fn new(args: RdfConfigArgs) -> Self {
        Self { args }
    }
}

impl Command for RdfConfigCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "rdf-config"
    }

    /// Executes the RdfConfig command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let format = self.args.format.into();
        let result_format = self.args.result_format.into();

        ctx.rudof.load_rdf_config(&self.args.input).with_format(&format).execute()?;

        ctx.rudof.serialize_rdf_config(&mut ctx.writer).with_format(&result_format).execute()?;

        Ok(())
    }
}
