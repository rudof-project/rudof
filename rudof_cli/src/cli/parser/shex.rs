use std::path::PathBuf;

use crate::cli::parser::CommonArgsNoBackend;
use crate::cli::wrappers::{DataReaderModeCli, ShExFormatCli, VizEngineCli};
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `shex` command
#[derive(Debug, Clone, Args)]
pub struct ShexArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema, FILE, URI or - for stdin. If omitted, shows the currently loaded schema",
        conflicts_with = "compiled_schema"
    )]
    pub schema: Option<InputSpec>,

    #[arg(
        long = "compiled-schema",
        value_name = "FILE",
        help = "Precompiled ShEx SchemaIR cache file, as produced by --compile-to. Loads it directly, \
                skipping parsing, imports and AST-to-IR compilation. Only -r/--result-format internal \
                (plus --statistics/--show-dependencies) can be used with it -- other result formats need \
                the original schema, loaded via --schema.",
        conflicts_with_all = ["schema", "schema_format", "base"]
    )]
    pub compiled_schema: Option<InputSpec>,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Schema format (ShExC, ShExJ, Turtle, ...), default = ShExC",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub schema_format: ShExFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result schema format",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub result_schema_format: ShExFormatCli,

    #[arg(
        long = "viz-engine",
        ignore_case = true,
        value_name = "ENGINE",
        help = "Visualization engine for image (SVG/PNG) result formats",
        default_value_t = VizEngineCli::PlantUml
    )]
    pub viz_engine: VizEngineCli,

    #[arg(short = 'l', long = "shape-label", value_name = "LABEL", help = "shape label")]
    pub shape: Option<String>,

    #[arg(short = 't', value_name = "BOOL", help = "Show processing time", long = "show-time")]
    pub show_time: Option<bool>,

    #[arg(
        long = "show-schema",
        help = "Show the loaded schema (default). Overridden by a later --no-show-schema.",
        default_value_t = true,
        action = clap::ArgAction::SetTrue,
        overrides_with = "no_show_schema"
    )]
    pub show_schema: bool,

    #[arg(
        long = "no-show-schema",
        help = "Don't show the loaded schema -- useful with -f binary to load a precompiled schema \
                without also trying (and failing) to render it in the default result format, which \
                needs the original schema. Overridden by a later --show-schema.",
        default_value_t = false,
        action = clap::ArgAction::SetTrue,
        overrides_with = "show_schema"
    )]
    pub no_show_schema: bool,

    #[arg(long = "statistics", value_name = "BOOL", help = "Show statistics about the schema")]
    pub show_statistics: Option<bool>,

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
    pub base: Option<String>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode (strict or lax)",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(
        long = "show-dependencies",
        value_name = "BOOL",
        help = "Show dependencies between shapes"
    )]
    pub show_dependencies: Option<bool>,

    #[arg(
        long = "compile-to",
        value_name = "FILE",
        help = "Compile the ShEx schema and write the precompiled SchemaIR cache to FILE.",
        conflicts_with = "compiled_schema"
    )]
    pub compile_to: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}
