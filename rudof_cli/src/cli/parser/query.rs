use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, QueryTypeCli, ResultQueryFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;
use std::path::PathBuf;

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

    #[arg(
        short = 'q',
        long = "query",
        value_name = "INPUT",
        help = "SPARQL query. If omitted, shows the results of the last query run"
    )]
    pub query: Option<InputSpec>,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result query format",
        default_value_t = ResultQueryFormatCli::Internal
    )]
    pub result_query_format: ResultQueryFormatCli,

    /// Run a Cypher query against a LadybugDB database instead of SPARQL
    ///
    /// The database is given by `--db` or by the connection details file
    /// written by `rudof connect`.
    #[arg(
        long = "cypher",
        value_name = "QUERY",
        help = "Cypher query to run against a LadybugDB database (requires --db or a connection file)",
        conflicts_with = "query"
    )]
    pub cypher: Option<String>,

    /// Path to the LadybugDB database directory (Cypher mode)
    #[arg(long = "db", value_name = "PATH", requires = "cypher")]
    pub db: Option<PathBuf>,

    /// Connection details file written by `rudof connect` (Cypher mode; default: .rudof-connection.toml)
    #[arg(long = "connection", value_name = "FILE", requires = "cypher")]
    pub connection: Option<PathBuf>,

    /// Open the database in read-only mode (Cypher mode)
    #[arg(long = "read-only")]
    pub read_only: bool,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
