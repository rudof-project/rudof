use crate::cli::parser::ShexArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Context, Result};
use rudof_lib::formats::ShExFormat;
use std::fs::File;
use std::io::BufWriter;

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
    ///
    /// With no `--schema`, there is nothing new to load, so this just
    /// re-serializes whatever schema is already loaded in the session
    /// (useful in the interactive shell, where state persists across
    /// commands).
    #[allow(clippy::unnecessary_fallible_conversions)]
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let schema_format = self.args.schema_format.into();
        let reader_mode = self.args.reader_mode.into();
        let result_schema_format: ShExFormat = self.args.result_schema_format.into();
        let viz_engine = self.args.viz_engine.into();
        // `--no-show-schema` overrides `--show-schema` (which defaults on)
        // when both are given -- see the flags' own docs for why a plain
        // `bool` field can't express "only the later flag wins" on its own.
        let show_schema = self.args.show_schema && !self.args.no_show_schema;

        if let Some(compiled_schema) = &self.args.compiled_schema {
            // `--compiled-schema` is a dedicated shortcut for the same
            // `ShExFormat::Binary` handling that `-f binary` drives.
            ctx.rudof
                .load_shex_schema(compiled_schema)
                .with_reader_mode(&reader_mode)
                .with_shex_schema_format(&ShExFormat::Binary)
                .execute()?;
        } else if let Some(schema) = &self.args.schema {
            let mut shex_schema_loading = ctx
                .rudof
                .load_shex_schema(schema)
                .with_reader_mode(&reader_mode)
                .with_shex_schema_format(&schema_format);
            if let Some(base) = &self.args.base {
                shex_schema_loading = shex_schema_loading.with_base(base);
            }
            shex_schema_loading.execute()?;
        }

        let mut shex_serialization = ctx
            .rudof
            .serialize_shex_schema(&mut ctx.writer)
            .with_show_schema(show_schema)
            .with_result_shex_format(&result_schema_format)
            .with_viz_engine(&viz_engine);

        if let Some(shape_label) = self.args.shape.as_deref() {
            shex_serialization = shex_serialization.with_shape(shape_label);
        }
        if let Some(show_statistics) = self.args.show_statistics {
            shex_serialization = shex_serialization.with_show_statistics(show_statistics);
        }
        if let Some(show_dependencies) = self.args.show_dependencies {
            shex_serialization = shex_serialization.with_show_dependencies(show_dependencies);
        }
        if let Some(show_time) = self.args.show_time {
            shex_serialization = shex_serialization.with_show_time(show_time);
        }
        shex_serialization.execute()?;

        if show_schema && matches!(result_schema_format, ShExFormat::Binary) {
            let destination = self
                .args
                .common
                .output
                .as_deref()
                .map_or_else(|| "stdout".to_string(), |path| path.display().to_string());
            report_shapes_saved(ctx, &destination);
        }

        if let Some(cache_path) = self.args.compile_to.as_deref() {
            // Likewise, `--compile-to` is a shortcut for
            // `serialize_shex_schema`'s `ShExFormat::Binary` result format --
            // the same one `-r binary` drives.
            let file = File::create(cache_path)
                .with_context(|| format!("Failed to create precompiled cache file '{}'", cache_path.display()))?;
            let mut writer = BufWriter::new(file);
            ctx.rudof
                .serialize_shex_schema(&mut writer)
                .with_result_shex_format(&ShExFormat::Binary)
                .execute()?;
            report_shapes_saved(ctx, &cache_path.display().to_string());
        }

        Ok(())
    }
}

/// Prints a short confirmation of how many shapes were compiled into a
/// `binary` result -- e.g. "1 shape(s) saved in target/user.bin". Printed to
/// stderr, never `ctx.writer`: for `-r binary`, `ctx.writer` may be the
/// binary cache file itself (via `-o`), and mixing text into it would
/// corrupt the cache.
fn report_shapes_saved(ctx: &CommandContext, destination: &str) {
    let shapes_count = ctx.rudof.shex_schema_shapes_count().unwrap_or(0);
    eprintln!("{shapes_count} shape(s) saved in {destination}");
}
