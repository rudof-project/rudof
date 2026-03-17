use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, ResultShaclValidationFormatCli,  ShaclFormatCli, ShaclValidationModeCli, ShaclValidationSortByModeCli};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `shacl-validate` command
#[derive(Debug, Clone, Args)]
pub struct ShaclValidateArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help= "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        long = "base-data",
        value_name = "IRI",
        help = "Base IRI (used to resolve relative IRIs in RDF data)"
    )]
    pub base_data: Option<String>,

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
        short = 's',
        long = "shapes",
        value_name = "INPUT",
        help = "Shapes graph: file, URI or -, if not set, it assumes the shapes come from the data"
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
        long = "base-shapes",
        value_name = "IRI",
        help = "Base IRI (used to resolve relative IRIs in Shapes)"
    )]
    pub base_shapes: Option<String>,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "ENDPOINT",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    /// Execution mode
    #[arg(
        short = 'm',
        long = "mode",
        value_name = "MODE",
        ignore_case = true,
        help = "Execution mode",
        default_value_t = ShaclValidationModeCli::Native,
        value_enum
    )]
    pub mode: ShaclValidationModeCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Ouput result format",
        default_value_t = ResultShaclValidationFormatCli::Details
    )]
    pub result_format: ResultShaclValidationFormatCli,

    #[arg(
        long = "sort_by",
        value_name = "SORT_MODE",
        ignore_case = true,
        help = "Sort result by",
        default_value_t = ShaclValidationSortByModeCli::Severity,
        value_enum
    )]
    pub sort_by: ShaclValidationSortByModeCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}