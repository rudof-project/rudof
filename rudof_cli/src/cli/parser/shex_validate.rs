use std::path::PathBuf;

use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{
    DataFormatCli, DataReaderModeCli, ResultShExValidationFormatCli, ShExFormatCli, ShExValidationSortByModeCli,
    ShapeMapFormatCli,
};
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `shex-validate` command
#[derive(Debug, Clone, Args)]
pub struct ShexValidateArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema file name, URI or - (for stdin)",
        required_unless_present = "list_external_resolvers"
    )]
    pub schema: Option<InputSpec>,

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
    pub shapemap: Option<InputSpec>,

    #[arg(
        long = "shapemap-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShapeMap format"
    )]
    pub shapemap_format: Option<ShapeMapFormatCli>,

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
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Ouput result format",
        default_value_t = ResultShExValidationFormatCli::Details
    )]
    pub result_format: ResultShExValidationFormatCli,

    #[arg(long = "map-state", value_name = "FILE", help = "MapState file name")]
    pub map_state: Option<PathBuf>,

    #[arg(
        long = "strict-iris",
        help = "Require <> brackets around IRIs (strict mode). By default bare http://… IRIs are accepted (lax mode)."
    )]
    pub strict_iris: bool,

    #[arg(
        long = "external-resolver",
        value_name = "SPEC",
        help = "External-shape resolver spec. Repeatable. Syntax: <kind>[:<arg>]. \
                Built-in kinds: 'reject-all', 'schema:<path>'. \
                Use --list-external-resolvers to enumerate.",
        action = clap::ArgAction::Append
    )]
    pub external_resolvers: Vec<String>,

    #[arg(
        long = "list-external-resolvers",
        help = "Print the available external-shape resolver kinds and exit"
    )]
    pub list_external_resolvers: bool,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
