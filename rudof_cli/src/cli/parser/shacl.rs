use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, ShaclFormatCli};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `shacl` command
#[derive(Debug, Clone, Args)]
pub struct ShaclArgs {
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
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "Endpoint",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    #[arg(
        short = 's',
        long = "shapes",
        value_name = "INPUT",
        help = "Shapes graph: File, URI or - for stdin, if not set, it assumes the shapes come from the data"
    )]
    pub shapes: Option<InputSpec>,

    #[arg(
        short = 'f',
        long = "shapes-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Shapes file format",
        default_value_t = ShaclFormatCli::Turtle
    )]
    pub shapes_format: ShaclFormatCli,

    #[arg(
        long = "base-data",
        value_name = "IRI",
        help = "Base RDF Data (used to resolve relative IRIs in RDF data)"
    )]
    pub base_data: Option<String>,

    #[arg(
        long = "base-shapes",
        value_name = "IRI",
        help = "Base RDF Data (used to resolve relative IRIs in Shapes)"
    )]
    pub base_shapes: Option<String>,

    #[arg(
        short = 'r',
        long = "result-shapes-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result shapes format",
        default_value_t = ShaclFormatCli::Internal
    )]
    pub result_shapes_format: ShaclFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}