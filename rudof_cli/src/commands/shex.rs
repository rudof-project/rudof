use crate::cli::parser::ShexArgs;
use crate::commands::base::{Command, CommandContext};
use crate::output::ColorSupport;
use anyhow::{Result, Context, bail};
use iri_s::MimeType;
use rudof_lib::{ReaderMode, ShExFormatter, rdf_reader_mode::RDFReaderMode};
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

    /// Configures the global Rudof settings based on command-line arguments.
    fn configure_runtime(&self, ctx: &mut CommandContext) {
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

    /// Loads and parses the ShEx schema into the current context.
    fn load_schema(&self, ctx: &mut CommandContext) -> Result<shex_ast::shex_format::ShExFormat> {
        let mime_type = self.args.result_schema_format.mime_type();
        let reader = self.args.schema.open_read(Some(mime_type), "ShexSchema")?;
        let base_iri = ctx.rudof.get_base_iri(&self.args.base)?;

        // Conversion logic for formats and modes
        let shex_format: rudof_lib::shex_format::ShExFormat = (&self.args.result_schema_format).into();
        let shex_format: shex_ast::shex_format::ShExFormat = shex_format.try_into()?;
        let reader_mode: ReaderMode = RDFReaderMode::from(&self.args.reader_mode).into();

        ctx.rudof.read_shex(
            reader,
            &shex_format,
            Some(base_iri.as_str()),
            &reader_mode,
            Some(&self.args.schema.source_name()),
        )?;

        Ok(shex_format)
    }

    /// Validates the schema's integrity, specifically checking for negative cycles.
    fn validate_schema(&self, ctx: &CommandContext) -> Result<()> {
        if ctx.rudof.config().shex_config().check_well_formed() {
            if let Some(shex_ir) = ctx.rudof.get_shex_ir() {
                if shex_ir.has_neg_cycle() {
                    bail!("Schema contains negative cycles: {:?}", shex_ir.neg_cycles());
                }
            }
        }
        Ok(())
    }

    /// Handles printing the schema or specific shapes to the output writer.
    fn handle_outputs(&self, ctx: &mut CommandContext, format: &shex_ast::shex_format::ShExFormat) -> Result<()> {
        let formatter = match ctx.color {
            ColorSupport::NoColor => ShExFormatter::default().without_colors(),
            _ => ShExFormatter::default(),
        };

        if let Some(shape_label) = &self.args.shape {
            let shape_selector = ctx.rudof.parse_shape_selector(shape_label)?;
            ctx.rudof.serialize_shape_current_shex(&shape_selector, format, &formatter, &mut ctx.writer)?;
        }

        if self.args.show_schema == Some(true) {
            ctx.rudof.serialize_current_shex(format, &formatter, &mut ctx.writer)?;
        }

        Ok(())
    }

    /// Displays the Internal Representation (IR) and detailed statistics if compilation is enabled.
    fn handle_compilation_details(&self, ctx: &mut CommandContext) -> Result<()> {
        if self.args.compile != Some(true) || !ctx.rudof.config().show_ir() {
            return Ok(());
        }

        let shex_ir = ctx.rudof.get_shex_ir()
            .context("Internal error: Schema was not compiled to IR")?;

        let mut out = io::stdout();
        writeln!(out, "ShEx Internal Representation:\n{shex_ir}")?;

        if ctx.rudof.config().show_extends() {
            for (key, value) in shex_ir.count_extends().iter() {
                writeln!(ctx.writer, "Shapes with {key} extends = {value}")?;
            }
        }

        if ctx.rudof.config().show_imports() {
            writeln!(ctx.writer, "Local shapes: {}/Total shapes {}", 
                shex_ir.local_shapes_count(), shex_ir.total_shapes_count())?;
        }

        if ctx.rudof.config().show_shapes() {
            for (label, source, _) in shex_ir.shapes() {
                let from_msg = if shex_ir.imported_schemas().is_empty() { 
                    String::new() 
                } else { 
                    format!(" from {source}") 
                };
                writeln!(ctx.writer, "{label}{from_msg}")?;
            }
        }

        if ctx.rudof.config().show_dependencies() {
            writeln!(ctx.writer, "\nDependencies:")?;
            for (source, posneg, target) in shex_ir.dependencies() {
                writeln!(out, "{source}-{posneg}->{target}")?;
            }
            writeln!(ctx.writer, "---end dependencies\n")?;
        }

        Ok(())
    }
}

impl Command for ShexCommand {
    fn name(&self) -> &'static str {
        "shex"
    }

    /// Orchestrates the ShEx command execution.
    /// 
    /// The logic follows these steps:
    /// 1. Start timer and initialize runtime configuration.
    /// 2. Load and parse the schema from the provided source.
    /// 3. Validate the structural integrity (well-formedness).
    /// 4. Output the schema/shapes as requested.
    /// 5. Show compilation metadata if the IR flag is active.
    /// 6. Report execution time.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let timer = Instant::now();

        // Step 1: Configuration
        self.configure_runtime(ctx);

        // Step 2: Ingestion
        let shex_format = self.load_schema(ctx)?;

        // Step 3: Validation
        self.validate_schema(ctx)?;

        // Step 4: Presentation
        self.handle_outputs(ctx, &shex_format)?;

        // Step 5: Compilation introspection
        self.handle_compilation_details(ctx)?;

        // Step 6: Telemetry
        if self.args.show_time == Some(true) {
            writeln!(io::stdout(), "elapsed: {:.03?} sec", timer.elapsed().as_secs_f64())?;
        }

        Ok(())
    }
}
