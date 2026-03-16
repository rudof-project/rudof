use crate::cli::parser::CompareArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Ok, Result};

/// Implementation of the `compare` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Compare command logic.
pub struct CompareCommand {
    /// Arguments specific to Compare command.
    args: CompareArgs,
}

impl CompareCommand {
    pub fn new(args: CompareArgs) -> Self {
        Self { args }
    }
}

impl Command for CompareCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "compare"
    }

    /// Executes the Compare command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        ctx.rudof.show_schema_comparison(
            &self.args.schema1, 
            &self.args.schema2, 
            self.args.base1.as_deref(), 
            self.args.base2.as_deref(), 
            Some(&self.args.reader_mode.into()), 
            &self.args.format1.into(), 
            &self.args.format2.into(), 
            &self.args.input_mode1.into(), 
            &self.args.input_mode2.into(), 
            self.args.shape1.as_deref(), 
            self.args.shape2.as_deref(), 
            self.args.show_time, 
            Some(&self.args.result_format.into()), 
            &mut ctx.writer
        );

        Ok(())
    }
}
