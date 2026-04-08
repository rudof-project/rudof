use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, ResultServiceFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `service` command
#[derive(Debug, Clone, Args)]
pub struct ServiceArgs {
    #[arg(short = 's', long = "service", value_name = "URL", help = "SPARQL service URL")]
    pub service: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "SPARQL service format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub service_format: DataFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Output result service format",
        default_value_t = ResultServiceFormatCli::Json
    )]
    pub result_service_format: ResultServiceFormatCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        help = "RDF Reader mode",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(
        long = "base",
        value_name = "IRI",
        help = "Base used to resolve relative IRIs in the service description"
    )]
    pub base_data: Option<String>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
