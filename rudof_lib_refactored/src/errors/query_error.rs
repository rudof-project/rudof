use thiserror::Error;

/// Errors that can occur when working with SPARQL queries.
#[derive(Error, Debug)]
pub enum QueryError {
    /// The SPARQL query result format is not supported by Rudof.
    #[error("Unsupported SPARQL query result format: '{format}'. Valid formats are: 'internal', 'turtle', 'ntriples', 'json-ld', 'rdf-xml', 'csv', 'trig', 'n3', 'nquads'")]
    UnsupportedResultQueryFormat { format: String },

    /// The SPARQL query type is not supported by Rudof.
    #[error("Unsupported SPARQL query type: '{query_type}'. Valid types are: 'select', 'construct', 'ask', 'describe'")]
    UnsupportedQueryType { query_type: String },
}