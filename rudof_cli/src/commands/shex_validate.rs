use crate::cli::parser::ShexValidateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use shacl_validation::validation_report::result;

/// Implementation of the `shex-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute ShexValidate logic.
pub struct ShexValidateCommand {
    /// Arguments specific to shex-validate.
    args: ShexValidateArgs,
}

impl ShexValidateCommand {
    pub fn new(args: ShexValidateArgs) -> Self {
        Self { args }
    }
}

impl Command for ShexValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shex-validate"
    }

    /// Executes the shex-validate logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let schema_format = self.args.schema_format.into();
        let shapemap_format = self.args.shapemap_format.into();
        let sort_order = self.args.sort_by.into();
        let result_format = self.args.result_format.into();

        let mut loading = ctx.rudof.load_data(&self.args.data).with_data_format(&data_format).with_reader_mode(&reader_mode);
        if let Some(base) = self.args.base_data.as_deref() { loading = loading.with_base(base); }
        if let Some(endpoint) = self.args.endpoint.as_deref() { loading = loading.with_endpoint(endpoint); }
        loading.execute()?;

        let mut shex_schema_loading = ctx.rudof.load_shex_schema(&self.args.schema)
            .with_reader_mode(&reader_mode)
            .with_schema_format(&schema_format);
        if let Some(base) = self.args.base_schema.as_deref() { shex_schema_loading  = shex_schema_loading.with_base_schema(base); }
        shex_schema_loading.execute()?;

        let mut shapemap_loading = ctx.rudof.load_shapemap(&self.args.shapemap).with_shapemap_format(&shapemap_format);
        if let Some(base_nodes) = self.args.base_data.as_deref() { shapemap_loading = shapemap_loading.with_base_nodes(base_nodes); }
        if let Some(base_shapes) = self.args.base_schema.as_deref() { shapemap_loading = shapemap_loading.with_base_shapes(base_shapes); }
        shapemap_loading.execute()?;

        ctx.rudof.validate_shex().execute()?;

        ctx.rudof.serialize_shex_validation_results(&mut ctx.writer)
            .with_sort_order(&sort_order)
            .with_result_format(&result_format)
            .execute()?;

        Ok(())
    }
}
