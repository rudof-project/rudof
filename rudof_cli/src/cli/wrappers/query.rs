use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::{QueryType, ResultQueryFormat};
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    QueryTypeCli,
    QueryType,
    {
        Select,
        Construct,
        Ask,
        Describe,
    }
);

cli_wrapper!(
    ResultQueryFormatCli,
    ResultQueryFormat,
    {
        Internal,
        Turtle,
        NTriples,
        JsonLd,
        Json,
        RdfXml,
        Csv,
        Markdown,
        AsciiTable,
        TriG,
        N3,
        NQuads,
    }
);

/// Query dialect: which language `-q`/`--query` is written in.
///
/// No corresponding `rudof_lib` type exists (Cypher execution is CLI-only,
/// see `commands/query.rs`), so this is a plain enum rather than a
/// `cli_wrapper!`.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[clap(rename_all = "lower")]
pub enum QueryDialectCli {
    /// SPARQL, run against the loaded RDF data or a SPARQL endpoint
    #[default]
    Sparql,
    /// Cypher, run against a LadybugDB database (see `rudof connect`)
    Cypher,
}

impl Display for QueryDialectCli {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let val = self.to_possible_value().expect("no skipped variants");
        write!(f, "{}", val.get_name())
    }
}
