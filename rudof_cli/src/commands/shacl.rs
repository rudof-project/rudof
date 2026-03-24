use crate::cli::parser::ShaclArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `shacl` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Shacl command logic.
pub struct ShaclCommand {
    /// Arguments specific to Shacl command.
    args: ShaclArgs,
}

impl ShaclCommand {
    pub fn new(args: ShaclArgs) -> Self {
        Self { args }
    }
}

impl Command for ShaclCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shacl"
    }

    /// Executes the Shacl command logic.
    #[allow(clippy::unnecessary_fallible_conversions)]
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let shacl_schema_format = self.args.shapes_format.into();
        let result_format = self.args.result_shapes_format.into();

        let mut loading = ctx.rudof.load_data()
            .with_data(&self.args.data)
            .with_data_format(&data_format)
            .with_reader_mode(&reader_mode);
        if let Some(base) = self.args.base_data.as_deref() { loading = loading.with_base(base); }
        if let Some(endpoint) = self.args.endpoint.as_deref() { loading = loading.with_endpoint(endpoint); }
        loading.execute()?;

        let mut loading_schema = ctx.rudof.load_shacl_shapes()
            .with_shacl_schema_format(&shacl_schema_format)
            .with_reader_mode(&reader_mode);
        if let Some(shacl_schema) = &self.args.shapes { loading_schema = loading_schema.with_shacl_schema(shacl_schema); }
        if let Some(base) = self.args.base_shapes.as_deref() { loading_schema = loading_schema.with_base(base); }
        loading_schema.execute()?;

        ctx.rudof.serialize_shacl_shapes(&mut ctx.writer).with_shacl_result_format(&result_format).execute()?;

        Ok(())
    }
}
