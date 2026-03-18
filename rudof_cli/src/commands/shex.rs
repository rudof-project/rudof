use crate::cli::parser::ShexArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `shex` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Shex logic.
pub struct ShexCommand {
    /// Arguments specific to shex.
    args: ShexArgs,
}

impl ShexCommand {
    pub fn new(args: ShexArgs) -> Self {
        Self { args }
    }
}

impl Command for ShexCommand {
    fn name(&self) -> &'static str {
        "shex"
    }

    /// Executes the ShEx command.
    #[allow(clippy::unnecessary_fallible_conversions)]
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let schema_format = self.args.schema_format.into();
        let reader_mode = self.args.reader_mode.into();
        let result_schema_format = self.args.result_schema_format.into();

        let mut shex_schema_loading = ctx.rudof.load_shex_schema(&self.args.schema)
            .with_reader_mode(&reader_mode)
            .with_schema_format(&schema_format);
        if let Some(base) = &self.args.base { shex_schema_loading = shex_schema_loading.with_base_schema(base); }
        shex_schema_loading.execute()?;

        let mut shex_serialization = ctx.rudof.serialize_shex_schema(&mut ctx.writer)
        .with_show_schema(self.args.show_schema)
        .with_schema_format(&result_schema_format);

        if let Some(shape_label) = self.args.shape.as_deref() { shex_serialization = shex_serialization.with_shape_label(shape_label); }
        if let Some(show_statistics) = self.args.show_statistics { shex_serialization = shex_serialization.with_show_statistics(show_statistics); }
        if let Some (show_dependencies) = self.args.show_dependencies { shex_serialization = shex_serialization.with_show_dependencies(show_dependencies); }
        if let Some(show_time) = self.args.show_time { shex_serialization = shex_serialization.with_show_time(show_time); }
        shex_serialization.execute()?;

        Ok(())
    }
}
