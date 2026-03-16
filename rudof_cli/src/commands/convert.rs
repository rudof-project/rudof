use crate::cli::parser::ConvertArgs;
use crate::commands::{
    base::{Command, CommandContext},
};
use anyhow::Result;

/// Implementation of the `convert` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Convert command logic.
pub struct ConvertCommand {
    /// Arguments specific to Convert command.
    args: ConvertArgs,
}

impl ConvertCommand {
    pub fn new(args: ConvertArgs) -> Self {
        Self { args }
    }
}

impl Command for ConvertCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "convert"
    }

    /// Executes the Convert command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        ctx.rudof.show_schema_conversion(
            &self.args.file, 
            self.args.base.as_deref(), 
            Some(&self.args.reader_mode.into()), 
            &self.args.input_mode.into(), 
            &self.args.output_mode.into(), 
            &self.args.format.into(), 
            &self.args.result_format.into(), 
            self.args.shape.as_deref(), 
            self.args.show_time, 
            &mut ctx.writer
        );

        Ok(())
    }
}
