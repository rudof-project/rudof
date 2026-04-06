use crate::cli::parser::ConvertArgs;
use crate::commands::base::{Command, CommandContext};
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
        let input_mode = self.args.input_mode.into();
        let output_mode = self.args.output_mode.into();
        let format = self.args.format.into();
        let result_format = self.args.result_format.into();
        let reader_mode = self.args.reader_mode.into();

        let mut conversion = ctx
            .rudof
            .show_schema_conversion(
                &self.args.file,
                &input_mode,
                &output_mode,
                &format,
                &result_format,
                &mut ctx.writer,
            )
            .with_reader_mode(&reader_mode);

        if let Some(base) = self.args.base.as_deref() {
            conversion = conversion.with_base(base);
        }
        if let Some(shape) = self.args.shape.as_deref() {
            conversion = conversion.with_shape(shape);
        }
        if let Some(show_time) = self.args.show_time {
            conversion = conversion.with_show_time(show_time);
        }
        if let Some(templates_folder) = self.args.template_folder.as_deref() {
            conversion = conversion.with_templates_folder(templates_folder);
        }
        if let Some(output_folder) = self.args.common.output.as_deref() {
            conversion = conversion.with_output_folder(output_folder);
        }

        conversion.execute()?;

        Ok(())
    }
}
