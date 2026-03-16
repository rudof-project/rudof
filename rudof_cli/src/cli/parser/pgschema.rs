use crate::cli::wrappers::PgSchemaFormatCli;
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `pgschema` command
#[derive(Debug, Clone, Args)]
pub struct PgschemaArgs {
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
        help = "PGSchema format",
        default_value_t = PgSchemaFormatCli::PgSchemaC,
        value_enum
    )]
    pub schema_format: PgSchemaFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result schema format",
        default_value_t = PgSchemaFormatCli::PgSchemaC,
        value_enum
    )]
    pub result_schema_format: PgSchemaFormatCli,

    #[arg(short = 't', value_name = "BOOL", help = "Show processing time", long = "show-time")]
    pub show_time: Option<bool>,

    #[arg(long = "show-schema", value_name = "BOOL", help = "Show schema")]
    pub show_schema: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}