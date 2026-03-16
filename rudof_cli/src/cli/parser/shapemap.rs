use crate::cli::wrappers::ShapeMapFormatCli;
use crate::cli::parser::CommonArgsOutputForceOverWrite;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `shapemap` command
#[derive(Debug, Clone, Args)]
pub struct ShapemapArgs {
    #[arg(
        short = 'm',
        long = "shapemap",
        value_name = "INPUT",
        help = "ShapeMap (FILE, URI or - for stdin)"
    )]
    pub shapemap: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShapeMap format, default = compact", 
        default_value_t = ShapeMapFormatCli::Compact
    )]
    pub shapemap_format: ShapeMapFormatCli,

    #[arg(long = "base-data", value_name = "IRI", help = "Base IRI for data")]
    pub base_data: Option<String>,

    #[arg(long = "base-schema", value_name = "IRI", help = "Base IRI for Schema")]
    pub base_schema: Option<String>,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Result shapemap format, default = compact",
        default_value_t = ShapeMapFormatCli::Compact
    )]
    pub result_shapemap_format: ShapeMapFormatCli,

    #[command(flatten)]
    pub common: CommonArgsOutputForceOverWrite,
}
