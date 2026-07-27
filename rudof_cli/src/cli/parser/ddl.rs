use crate::cli::parser::CommonArgsOutputForceOverWrite;
use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, DdlDialectCli};
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `ddl` command
///
/// Derives a property graph schema from RDF data and generates the DDL
/// needed to materialize it in a property graph database, without requiring
/// or touching any database (see discussion #747).
#[derive(Debug, Clone, Args)]
pub struct DdlArgs {
    /// RDF data files used to derive the property graph schema
    #[clap(value_parser = clap::value_parser!(InputSpec), required = true)]
    pub data: Vec<InputSpec>,

    /// DDL dialect to generate
    #[arg(
        short = 'd',
        long = "dialect",
        value_name = "DIALECT",
        ignore_case = true,
        help = "DDL dialect: cypher | gql",
        default_value_t = DdlDialectCli::Cypher,
        value_enum
    )]
    pub dialect: DdlDialectCli,

    /// Name of the graph type in generated GQL DDL
    #[arg(
        long = "graph-type-name",
        value_name = "NAME",
        help = "Graph type name used by the gql dialect",
        default_value = "rudof_graph"
    )]
    pub graph_type_name: String,

    /// RDF data format
    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    /// Base IRI for data
    #[arg(long = "base-data", value_name = "IRI")]
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

    #[command(flatten)]
    pub common: CommonArgsOutputForceOverWrite,
}
