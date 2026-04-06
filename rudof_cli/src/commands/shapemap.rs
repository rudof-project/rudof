use crate::cli::parser::ShapemapArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `shapemap` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Shapemap logic.
pub struct ShapemapCommand {
    /// Arguments specific to shapemap.
    args: ShapemapArgs,
}

impl ShapemapCommand {
    pub fn new(args: ShapemapArgs) -> Self {
        Self { args }
    }
}

impl Command for ShapemapCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shapemap"
    }

    /// Executes the shapemap logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let format = self.args.shapemap_format.into();
        let result_format = self.args.result_shapemap_format.into();

        let mut shapemap_loading = ctx
            .rudof
            .load_shapemap(&self.args.shapemap)
            .with_shapemap_format(&format);
        if let Some(base_data) = self.args.base_data.as_deref() {
            shapemap_loading = shapemap_loading.with_base_nodes(base_data);
        }
        if let Some(base_schema) = self.args.base_schema.as_deref() {
            shapemap_loading = shapemap_loading.with_base_shapes(base_schema);
        }
        shapemap_loading.execute()?;

        ctx.rudof
            .serialize_shapemap(&mut ctx.writer)
            .with_result_shapemap_format(&result_format)
            .execute()?;

        Ok(())
    }
}
