use crate::cli::wrappers::{
    DataFormatCli, DataReaderModeCli, ResultValidationFormatCli, ShExFormatCli,ShaclValidationModeCli, ShapeMapFormatCli, ValidationModeCli, ValidationSortByModeCli
};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `validate` command
#[derive(Debug, Clone, Args)]
pub struct ValidateArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(short = 'M', long = "mode",
        value_name = "MODE",
        ignore_case = true,
        help = "Validation mode (ShEx or SHACL)",
        default_value_t = ValidationModeCli::ShEx
    )]
    pub validation_mode: ValidationModeCli,

    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema used for validatio, FILE, URI or - for stdin"
    )]
    pub schema: Option<InputSpec>,

    #[arg(
        short = 'f',
        long = "schema-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Schema format",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub schema_format: ShExFormatCli,

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

    #[arg(long = "base-data", value_name = "IRI", help = "Base IRI for data")]
    pub base_data: Option<String>,

    #[arg(long = "base-schema", value_name = "IRI", help = "Base IRI for Schema")]
    pub base_schema: Option<String>,

    #[arg(
        long = "sort_by",
        value_name = "SORT_MODE",
        ignore_case = true,
        help = "Sort result by (default = node)",
        default_value_t = ValidationSortByModeCli::Node,
        value_enum
    )]
    pub sort_by: ValidationSortByModeCli,

    #[arg(short = 'n', long = "node", value_name = "NODE", help = "Node to validate")]
    pub node: Option<String>,

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
        value_name = "FORMAT",
        ignore_case = true,
        help = "RDF Data format (default = turtle)",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "ENDPOINT",
        help = "Endpoint with RDF data"
    )]
    pub endpoint: Option<String>,

    #[arg(
        long = "max-steps",
        value_name = "NUMBER",
        help = "max steps to run during validation",
        default_value_t = 100
    )]
    pub max_steps: usize,

    #[arg(
        short = 'S',
        long = "shacl-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "SHACL validation mode (default = native)",
        default_value_t = ShaclValidationModeCli::Native,
        value_enum
    )]
    pub shacl_validation_mode: ShaclValidationModeCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE", help = "RDF Reader mode",
        ignore_case = true,
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT", help = "Ouput result format, default = compact",
        default_value_t = ResultValidationFormatCli::Compact
    )]
    pub result_format: ResultValidationFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}