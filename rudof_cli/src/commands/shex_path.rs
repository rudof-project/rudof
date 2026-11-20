use crate::cli::parser::ShexPathArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `shex-path` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to evaluate a
/// [ShExPath](https://shexspec.github.io/spec/ShExPath) against a ShEx schema,
/// writing the selected schema elements as a JSON array of
/// `{"location": ..., "value": ...}` objects.
pub struct ShexPathCommand {
    /// Arguments specific to the ShexPath command.
    args: ShexPathArgs,
}

impl ShexPathCommand {
    pub fn new(args: ShexPathArgs) -> Self {
        Self { args }
    }
}

impl Command for ShexPathCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shex-path"
    }

    /// Executes the ShexPath command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let format = self.args.schema_format.into();

        let mut loading = ctx
            .rudof
            .load_shex_schema(&self.args.schema)
            .with_shex_schema_format(&format);
        if let Some(base) = &self.args.base {
            loading = loading.with_base(base);
        }
        loading.execute()?;

        let matches = ctx.rudof.evaluate_shex_path(&self.args.path).execute()?;
        serde_json::to_writer_pretty(&mut ctx.writer, &matches)?;
        writeln!(ctx.writer)?;

        Ok(())
    }
}
