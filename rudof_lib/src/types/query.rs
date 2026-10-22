use rudof_rdf::rdf_core::query::QuerySolutions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sparql_service::RdfData;

#[derive(Debug, Clone)]
pub enum QueryResult {
    Select(QuerySolutions<RdfData>),
    Construct(String),
    Ask(bool),
}

impl QueryResult {
    /// Which kind of result this is, without having to match on it.
    pub fn kind(&self) -> QueryResultKind {
        match self {
            QueryResult::Select(_) => QueryResultKind::Select,
            QueryResult::Construct(_) => QueryResultKind::Construct,
            QueryResult::Ask(_) => QueryResultKind::Ask,
        }
    }
}

/// The kind of result a SPARQL query produces, which follows from its query form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum QueryResultKind {
    /// Variable bindings, from a `SELECT`.
    Select,
    /// An RDF graph, from a `CONSTRUCT`.
    Construct,
    /// A boolean, from an `ASK`.
    Ask,
}
