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
        let format1 = self.args.format1.into();
        let format2 = self.args.format2.into();
        let input_mode1 = self.args.input_mode1.into();
        let input_mode2 = self.args.input_mode2.into();
        let result_format = self.args.result_format.into();

        let mut comparison = ctx.rudof.show_schema_comparison(
            &self.args.schema1, 
            &self.args.schema2,
            &format1, 
            &format2,
            &input_mode1, 
            &input_mode2,
            &mut ctx.writer
        ).with_result_format(&result_format);

        if let Some(base1) = self.args.base1.as_deref() { comparison = comparison.with_base1(base1); }
        if let Some(base2) = self.args.base2.as_deref() { comparison = comparison.with_base2(base2); }
        if let Some(shape1) = self.args.shape1.as_deref() { comparison = comparison.with_shape1(shape1); }
        if let Some(shape2) = self.args.shape2.as_deref() { comparison = comparison.with_shape2(shape2); }
        if let Some(show_time) = self.args.show_time { comparison = comparison.with_show_time(show_time); }
        
        comparison.execute()?;

        Ok(())
    }
}
