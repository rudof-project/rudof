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

    /// Errors related to specifying the data source.
    #[error("Data source specification error: {message}")]
    DataSourceSpec { message: String },

    /// Failed to parse a SPARQL query.
    #[error("Failed to parse SPARQL query from '{source_name}': {error}")]
    FailedParsingQuery {
        source_name: String,
        error: String,
    },

    /// No SPARQL query loaded when attempting to serialize.
    #[error("No SPARQL query loaded. Please load a SPARQL query before attempting to serialize.")]
    NoQueryLoaded,

    /// Errors that occur during SPARQL query serialization.
    #[error("Failed to serialize SPARQL query: {error}")]
    FailedSerializingQuery {
        error: String,
    },

    /// Errors that occur during SPARQL query execution.
    #[error("Failed to execute {query_type} query: {error}")]
    FailedExecutingQuery {
        query_type: String,
        error: String,
    },

    /// No query results available when attempting to serialize.
    #[error("No query results available. Please run a SPARQL query before attempting to serialize results.")]
    NoQueryResultsAvailable,

    /// Errors that occur during SPARQL query results serialization.
    #[error("Failed to serialize SPARQL query results: {error}")]
    FailedSerializingQueryResults {
        error: String,
    },
}