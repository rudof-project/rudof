use crate::cli::parser::ShaclValidateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use shacl_validation::validation_report::result;

/// Implementation of the `shacl-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Shacl Validate logic.
pub struct ShaclValidateCommand {
    /// Arguments specific to shacl-validate.
    args: ShaclValidateArgs,
}

impl ShaclValidateCommand {
    pub fn new(args: ShaclValidateArgs) -> Self {
        Self { args }
    }
}

impl Command for ShaclValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shacl-validate"
    }

    /// Executes the shacl-validate logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let shacl_schema_format = self.args.shapes_format.into();
        let shacl_validation_mode = self.args.mode.into();
        let sort_order = self.args.sort_by.into();
        let result_format = self.args.result_format.into();

        let mut loading = ctx.rudof.load_data(&self.args.data).with_data_format(&data_format).with_reader_mode(&reader_mode);
        if let Some(base) = self.args.base_data.as_deref() { loading = loading.with_base(base); }
        if let Some(endpoint) = self.args.endpoint.as_deref() { loading = loading.with_endpoint(endpoint); }
        loading.execute()?;

        let mut loading_schema = ctx.rudof.load_shacl_schema()
            .with_schema_format(&shacl_schema_format)
            .with_reader_mode(&reader_mode);
        if let Some(shacl_schema) = &self.args.shapes { loading_schema = loading_schema.with_schema(shacl_schema); }
        if let Some(base) = self.args.base_shapes.as_deref() { loading_schema = loading_schema.with_base(base); }
        loading_schema.execute()?;

        ctx.rudof.validate_shacl().with_mode(&shacl_validation_mode).execute()?;

        ctx.rudof.serialize_shacl_validation_results(&mut ctx.writer)
            .with_sort_order(&sort_order)
            .with_result_format(&result_format)
            .execute()?;

        Ok(())
    }
}
