use crate::cli::wrappers::{DataFormatCli, ResultPgSchemaValidationFormatCli, ShapeMapFormatCli};
use crate::cli::parser::CommonArgsOutputForceOverWrite;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `pgschema-validate` command
#[derive(Debug, Clone, Args)]
pub struct PgSchemaValidateArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "PGSchema file, URI or - (for stdin)"
    )]
    pub schema: Option<InputSpec>,

    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Property Graph data format",
        default_value_t = DataFormatCli::Pg
    )]
    pub data_format: DataFormatCli,

    #[arg(
        short = 'm',
        long = "shapemap",
        value_name = "INPUT",
        help = "ShapeMap used for validation, FILE, URI or - for stdin"
    )]
    pub shapemap: Option<InputSpec>,

    #[arg(
        long = "shapemap-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShapeMap format",
        default_value_t = ShapeMapFormatCli::Compact,
    )]
    pub shapemap_format: ShapeMapFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Output result format",
        default_value_t = ResultPgSchemaValidationFormatCli::Compact
    )]
    pub result_validation_format: ResultPgSchemaValidationFormatCli,

    #[command(flatten)]
    pub common: CommonArgsOutputForceOverWrite,
}