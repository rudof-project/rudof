use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, QueryDialectCli, QueryTypeCli, ResultQueryFormatCli};
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

    /// Query to run: a file, a URL, `-` for stdin, or the query text itself.
    /// SPARQL by default; give `--dialect cypher` for a Cypher query against
    /// a LadybugDB database instead. If omitted, shows the results of the
    /// last query run (SPARQL only).
    #[arg(
        short = 'q',
        long = "query",
        value_name = "INPUT",
        help = "Query to run (SPARQL by default, or Cypher with --dialect cypher). If omitted, shows the results of the last query run"
    )]
    pub query: Option<InputSpec>,

    /// Query dialect/language: `sparql` (default) or `cypher`.
    #[arg(
        long = "dialect",
        value_name = "DIALECT",
        ignore_case = true,
        help = "Query dialect: sparql (default) | cypher",
        default_value_t = QueryDialectCli::Sparql,
        value_enum
    )]
    pub dialect: QueryDialectCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result query format",
        default_value_t = ResultQueryFormatCli::Internal
    )]
    pub result_query_format: ResultQueryFormatCli,

    /// Path to the LadybugDB database directory. Only meaningful with
    /// `--dialect cypher`.
    #[arg(long = "db", value_name = "PATH")]
    pub db: Option<PathBuf>,

    /// Connection details file written by `rudof connect` (default:
    /// .rudof-connection.toml). Only meaningful with `--dialect cypher`.
    #[arg(long = "connection", value_name = "FILE")]
    pub connection: Option<PathBuf>,

    /// Open the database in read-only mode. Only meaningful with `--dialect cypher`.
    #[arg(long = "read-only")]
    pub read_only: bool,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
