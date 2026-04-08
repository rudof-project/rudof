use crate::cli::parser::CommonArgsOutputForceOverWrite;
use crate::cli::wrappers::{DataFormatCli, ResultPgSchemaValidationFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `pgschema-validate` command
#[derive(Debug, Clone, Args)]
pub struct PgSchemaValidateArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "PGSchema file, URI or - (for stdin)"
    )]
    pub schema: InputSpec,

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
        long = "typemap",
        value_name = "INPUT",
        help = "Type map used for validation, FILE, URI or - for stdin"
    )]
    pub typemap: InputSpec,

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
