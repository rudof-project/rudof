use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, ResultShExValidationFormatCli, ShExFormatCli, ShapeMapFormatCli, ShExValidationSortByModeCli};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `shex-validate` command
#[derive(Debug, Clone, Args)]
pub struct ShexValidateArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema file name, URI or - (for stdin)"
    )]
    pub schema: InputSpec,

    #[arg(
        short = 'f',
        long = "schema-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "ShEx Schema format",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub schema_format: ShExFormatCli,

    #[arg(short = 'm', long = "shapemap", value_name = "INPUT", help = "ShapeMap")]
    pub shapemap: InputSpec,

    #[arg(
        long = "shapemap-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShapeMap format",
        default_value_t = ShapeMapFormatCli::Compact,
    )]
    pub shapemap_format: ShapeMapFormatCli,

    #[arg(short = 'n', long = "node", value_name = "NODE", help = "Node to validate")]
    pub node: Option<String>,

    #[arg(
        long = "sort_by",
        value_name = "SORT_MODE",
        ignore_case = true,
        help = "Sort result by (default = node)",
        default_value_t = ShExValidationSortByModeCli::Node,
        value_enum
    )]
    pub sort_by: ShExValidationSortByModeCli,

    #[arg(
        short = 'l',
        long = "shape-label",
        value_name = "LABEL",
        help = "shape label (default = START)",
        group = "node_shape"
    )]
    pub shape: Option<String>,

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
        long = "base-schema",
        value_name = "IRI",
        help = "Base Schema (used to resolve relative IRIs in Schema)"
    )]
    pub base_schema: Option<String>,

    #[arg(
        long = "base-data",
        value_name = "IRI",
        help = "Base RDF Data IRI (used to resolve relative IRIs in RDF data)"
    )]
    pub base_data: Option<String>,

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
        value_name = "NAME",
        help = "Endpoint with RDF data (name or URL)"
    )]
    pub endpoint: Option<String>,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Ouput result format",
        default_value_t = ResultShExValidationFormatCli::Details
    )]
    pub result_format: ResultShExValidationFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}