use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, ResultDataFormatCli};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `shacl-validate` command
#[derive(Debug, Clone, Args)]
pub struct DataArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
    pub base: Option<String>,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "Endpoint",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    /// RDF Reader mode
    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Ouput result format",
        default_value_t = ResultDataFormatCli::Turtle
    )]
    pub result_format: ResultDataFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
