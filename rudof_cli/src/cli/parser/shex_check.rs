use crate::cli::parser::CommonArgsNoBackend;
use crate::cli::wrappers::ShExFormatCli;
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `shex-check` command
#[derive(Debug, Clone, Args)]
pub struct ShexCheckArgs {
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

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
    pub base: Option<String>,

    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}
