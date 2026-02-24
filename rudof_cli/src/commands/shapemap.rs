use crate::cli::parser::ShapemapArgs;
use crate::commands::base::{Command, CommandContext};
use crate::output::ColorSupport;
use anyhow::Result;
use rudof_lib::{shapemap_format::ShapeMapFormat, ShapeMapFormatter, ShapeMapFormat as ShexAstShapeMapFormat};

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
        // Convert CLI types to library types
        let reader = self.args.shapemap.open_read(None, "ShapeMap")?;
        let result_format: ShapeMapFormat = (&self.args.result_shapemap_format).into();
        let result_format: ShexAstShapeMapFormat = result_format.into();
        let shapemap_format: ShapeMapFormat = (&self.args.shapemap_format).into();
        let shapemap_format: ShexAstShapeMapFormat = shapemap_format.into();
        let formatter = match ctx.color {
            ColorSupport::NoColor => ShapeMapFormatter::default().without_colors(),
            _ => ShapeMapFormatter::default(),
        };

        // Load shapemap into rudof
        ctx.rudof.read_shapemap(reader, self.args.shapemap.source_name().as_str(), &shapemap_format)?;

        // Write results in the requested format
        ctx.rudof.serialize_shapemap(&result_format, &formatter, &mut ctx.writer)?;

        Ok(())
    }
}
