use crate::cli::wrappers::{DataReaderModeCli, ShExFormatCli};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `shex` command
#[derive(Debug, Clone, Args)]
pub struct ShexArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema, FILE, URI or - for stdin"
    )]
    pub schema: InputSpec,

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
        default_value_t = ShExFormatCli::ShExJ
    )]
    pub result_schema_format: ShExFormatCli,

    #[arg(short = 'l', long = "shape-label", value_name = "LABEL", help = "shape label")]
    pub shape: Option<String>,

    #[arg(short = 't', value_name = "BOOL", help = "Show processing time", long = "show-time")]
    pub show_time: Option<bool>,

    #[arg(
        long = "show-schema", 
        default_value_t = true,
        action = clap::ArgAction::SetTrue,
        overrides_with = "no_show_schema"
    )]
    pub show_schema: bool,

    #[arg(
        long = "no-show-schema", 
        action = clap::ArgAction::SetFalse,
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
        long = "compile",
        value_name = "BOOL",
        help = "Compile Schema to Internal representation"
    )]
    pub compile: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}