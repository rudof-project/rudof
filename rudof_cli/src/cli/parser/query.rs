use crate::cli::wrappers::{DataFormatCli, QueryTypeCli, DataReaderModeCli, ResultQueryFormatCli};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `query` command
#[derive(Debug, Clone, Args)]
pub struct QueryArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        short = 'b',
        long = "base",
        value_name = "IRI",
        help = "Base IRI (used to resolve relative IRIs in RDF data)"
    )]
    pub base: Option<String>,

    #[arg(long = "query-type",
        value_name = "TYPE",
        ignore_case = true,
        help = "Query type (SELECT, ASK, CONSTRUCT, DESCRIBE)",
        default_value_t = QueryTypeCli::Select,
        value_enum
    )]
    pub query_type: QueryTypeCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = DataReaderModeCli::Strict,
            value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(short = 'q', long = "query", value_name = "INPUT", help = "SPARQL query")]
    pub query: InputSpec,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "Endpoint",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result query format",
        default_value_t = ResultQueryFormatCli::Internal
    )]
    pub result_query_format: ResultQueryFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}