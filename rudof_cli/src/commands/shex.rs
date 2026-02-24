use crate::cli::parser::ShexArgs;
use crate::commands::base::{Command, CommandContext};
use crate::output::ColorSupport;
use anyhow::Result;
use rudof_lib::{ReaderMode, ShExFormatter, rdf_reader_mode::RDFReaderMode, ShExFormat as ShExAstShExFormat, shex_format::ShExFormat};
use std::{
    io::{self, Write},
    time::Instant,
};

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

    /// Configures the ShEx-specific runtime settings.
    fn configure_shex_settings(&self, ctx: &mut CommandContext) {
        if let Some(show_dependencies) = self.args.show_dependencies {
            ctx.rudof
                .config()
                .shex_config()
                .with_show_dependencies(show_dependencies);
        }

        if let Some(flag) = self.args.show_statistics {
            ctx.rudof.config().shex_config().set_show_extends(flag);
        }
    }

    /// Writes a specific shape to the output if requested.
    fn write_shape_if_requested(
        &self,
        ctx: &mut CommandContext,
        format: &shex_ast::shex_format::ShExFormat,
    ) -> Result<()> {
        if let Some(shape_label) = &self.args.shape {
            let formatter = Self::get_formatter(&ctx.color);
            ctx.rudof
                .serialize_shape_by_label(shape_label, format, &formatter, &mut ctx.writer)?;
        }
        Ok(())
    }

    /// Writes the entire schema to the output if requested.
    fn write_schema_if_requested(
        &self,
        ctx: &mut CommandContext,
        format: &shex_ast::shex_format::ShExFormat,
    ) -> Result<()> {
        if self.args.show_schema == Some(true) {
            let formatter = Self::get_formatter(&ctx.color);
            ctx.rudof.serialize_current_shex(format, &formatter, &mut ctx.writer)?;
        }
        Ok(())
    }

    /// Displays compilation details including IR, statistics, and dependencies.
    fn write_compilation_details(&self, ctx: &mut CommandContext) -> Result<()> {
        if self.args.compile != Some(true) || !ctx.rudof.config().show_ir() {
            return Ok(());
        }

        let stats = ctx.rudof.get_shex_statistics()?;
        let mut out = io::stdout();

        // Show IR representation
        if let Some(shex_ir) = ctx.rudof.get_shex_ir() {
            writeln!(out, "ShEx Internal Representation:\n{shex_ir}")?;
        }

        // Show extends statistics
        if ctx.rudof.config().show_extends() {
            for (key, value) in stats.extends_count.iter() {
                writeln!(ctx.writer, "Shapes with {key} extends = {value}")?;
            }
        }

        // Show import statistics
        if ctx.rudof.config().show_imports() {
            writeln!(
                ctx.writer,
                "Local shapes: {}/Total shapes {}",
                stats.local_shapes_count, stats.total_shapes_count
            )?;
        }

        // Show shape labels
        if ctx.rudof.config().show_shapes() {
            for (label, source, _) in &stats.shapes {
                let from_msg = if stats.has_imports {
                    format!(" from {source}")
                } else {
                    String::new()
                };
                writeln!(ctx.writer, "{label}{from_msg}")?;
            }
        }

        // Show dependencies
        if ctx.rudof.config().show_dependencies() {
            writeln!(ctx.writer, "\nDependencies:")?;
            for (source, posneg, target) in &stats.dependencies {
                writeln!(out, "{source}-{posneg}->{target}")?;
            }
            writeln!(ctx.writer, "---end dependencies\n")?;
        }

        Ok(())
    }

    /// Gets the appropriate formatter based on color support.
    fn get_formatter(color: &ColorSupport) -> ShExFormatter {
        match color {
            ColorSupport::NoColor => ShExFormatter::default().without_colors(),
            _ => ShExFormatter::default(),
        }
    }
}

impl Command for ShexCommand {
    fn name(&self) -> &'static str {
        "shex"
    }

    /// Executes the ShEx command.
    ///
    /// The workflow:
    /// 1. Configure runtime settings
    /// 2. Load and parse the schema
    /// 3. Validate schema well-formedness
    /// 4. Output requested information (shape, schema, statistics)
    /// 5. Show timing information if requested
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let timer = Instant::now();

        // Configure runtime
        self.configure_shex_settings(ctx);

        // Load schema
        let schema_format: ShExFormat = (&self.args.schema_format).into();
        let schema_format: ShExAstShExFormat = schema_format.try_into()?;
        let result_format: ShExFormat  = (&self.args.result_schema_format).into();
        let result_format: ShExAstShExFormat = result_format.try_into()?;
        let reader_mode: ReaderMode = RDFReaderMode::from(&self.args.reader_mode).into();

        ctx.rudof.load_shex_schema(
            &self.args.schema,
            &schema_format.try_into()?,
            &self.args.base,
            &reader_mode,
        )?;

        // Validate well-formedness
        ctx.rudof.validate_shex_schema_well_formed()?;

        // Output information
        self.write_shape_if_requested(ctx, &result_format)?;
        self.write_schema_if_requested(ctx, &result_format)?;
        self.write_compilation_details(ctx)?;

        // Show timing
        if self.args.show_time == Some(true) {
            writeln!(io::stdout(), "elapsed: {:.03?} sec", timer.elapsed().as_secs_f64())?;
        }

        Ok(())
    }
}
