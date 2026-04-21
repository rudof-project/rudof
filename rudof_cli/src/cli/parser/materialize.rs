use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{DataReaderModeCli, ResultDataFormatCli, ShExFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;
use std::path::PathBuf;

/// Arguments for the `materialize` command
#[derive(Debug, Clone, Args)]
pub struct MaterializeArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "ShEx schema, FILE, URI or - for stdin"
    )]
    pub schema: InputSpec,

    #[arg(
        short = 'f',
        long = "schema-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShEx schema format (ShExC, ShExJ, ...), default = ShExC",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub schema_format: ShExFormatCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF reader mode (strict or lax)",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI for the schema")]
    pub base: Option<String>,

    #[arg(
        short = 'm',
        long = "map-state",
        value_name = "FILE",
        help = "JSON file containing the MapState produced by ShEx validation with Map semantic actions"
    )]
    pub map_state: Option<PathBuf>,

    #[arg(
        short = 'n',
        long = "node",
        value_name = "IRI",
        help = "IRI of the root subject node; a fresh blank node is used when omitted"
    )]
    pub node: Option<String>,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "RDF output format for the materialized graph (Turtle, NTriples, ...)",
        default_value_t = ResultDataFormatCli::Turtle
    )]
    pub result_format: ResultDataFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
