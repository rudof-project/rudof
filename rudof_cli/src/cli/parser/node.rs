use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, NodeInspectionModeCli};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;

/// Arguments for the `node` command
#[derive(Debug, Clone, Args)]
pub struct NodeArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 'n',
        long = "node",
        value_name = "Node",
        help = "Node to show information (can be a URI or prefixed name)"
    )]
    pub node: String,

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
        short = 'e',
        long = "endpoint",
        value_name = "Endpoint",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
    pub base: Option<String>,

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
        short = 'm',
        long = "show-node-mode",
        ignore_case = true,
        value_name = "MODE",
        help = "Mode used to show the node information",
        default_value_t = NodeInspectionModeCli::Outgoing
    )]
    pub show_node_mode: NodeInspectionModeCli,

    #[arg(long = "show-hyperlinks", help = "Show hyperlinks in the output")]
    pub show_hyperlinks: Option<bool>,

    #[arg(
        short = 'p',
        long = "predicates",
        value_name = "PREDICATES",
        help = "List of predicates to show"
    )]
    pub predicates: Option<Vec<String>>,

    #[arg(
        short = 'd',
        long = "depth",
        value_name = "NUMBER",
        help = "outgoing number of levels, default = 1",
        default_value_t = 1
    )]
    pub depth: usize,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
