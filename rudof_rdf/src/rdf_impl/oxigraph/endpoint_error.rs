use crate::rdf_impl::SparqlVars;
use oxiri::IriParseError;
use oxrdf::Term;
use rudof_iri::error::IriSError;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with SPARQL endpoints.
#[derive(Error, Debug)]
pub enum OxigraphEndpointError {
    /// Error parsing a SPARQL query string.
    ///
    /// # Fields
    /// - `query_str`: The SPARQL query string that failed to parse
    /// - `error`: Detailed description of the parsing failure
    #[error("SPARQL parse error: {error}, query:\n{query_str}")]
    SPARQLParseError { query_str: String, error: String },

    /// Error when a CONSTRUCT query uses an unsupported result format.
    ///
    /// # Fields
    /// - `format`: The unsupported format identifier
    #[error("Unsupported format for CONSTRUCT query: {format:?}")]
    UnsupportedConstructFormat { format: String },

    /// Error making an HTTP request to the SPARQL endpoint.
    ///
    /// # Fields
    /// - `e`: The underlying reqwest error
    #[error("HTTP Request error: {e:?}")]
    HTTPRequestError { e: reqwest::Error },

    /// Error parsing a URL for the SPARQL endpoint.
    ///
    /// # Fields
    /// - `e`: The underlying URL parsing error
    #[error("URL parser error: {e:?}")]
    URLParseError { e: url::ParseError },

    /// Error parsing SPARQL query results.
    ///
    /// # Fields
    /// - `e`: The underlying SPARQL results parsing error
    #[error("SPARQL Results parser: {e:?}")]
    SPAResults { e: sparesults::QueryResultsParseError },

    /// Error parsing an IRI.
    ///
    /// # Fields
    /// - `err`: The underlying IRI parsing error
    #[error(transparent)]
    IriParseError {
        #[from]
        err: IriParseError,
    },

    /// Error when an unknown endpoint name is referenced.
    ///
    /// # Fields
    /// - `name`: The unknown endpoint name
    #[error("Unknown name for endpoint: {name}")]
    UnknownEndpointName { name: String },

    /// Error parsing an [`crate::rdf_impl::EndpointStrategy`] from a string.
    ///
    /// # Fields
    /// - `name`: The unrecognized strategy name
    #[error("Unknown endpoint strategy '{name}': expected 'sparql' or 'dereference'")]
    UnknownEndpointStrategy { name: String },

    /// Error parsing the response body from the endpoint.
    ///
    /// # Fields
    /// - `body`: The body content that failed to parse
    #[error("Error parsing body: {body}")]
    ParsingBody { body: String },

    /// A SPARQL request was aborted because cancellation was requested
    /// (e.g. Ctrl-C in the interactive shell) while it was in flight or
    /// waiting to retry.
    #[error("SPARQL request cancelled")]
    Cancelled,

    /// Error when a SPARQL solution contains a non-IRI value where an IRI was expected.
    ///
    /// # Fields
    /// - `value`: The term that is not an IRI
    #[error("SPARQL solutions error: Expected IRI, got {value}")]
    SPARQLSolutionErrorNoIRI { value: Term },

    /// Error when required variables are not found in a SPARQL solution.
    ///
    /// # Fields
    /// - `vars`: The variables that were expected
    /// - `solution`: String representation of the solution
    #[error("SPARQL solutions error: Not found vars {vars} in solution {solution:?}")]
    NotFoundVarsInSolution { vars: SparqlVars, solution: String },

    /// Error when a SPARQL solution contains a non-subject value where a subject was expected.
    ///
    /// # Fields
    /// - `value`: The term that is not a subject
    #[error("SPARQL solutions error: Expected Subject, got {value}")]
    SPARQLSolutionErrorNoSubject { value: Term },

    /// Error when a value is not found in a SPARQL solution.
    ///
    /// # Fields
    /// - `value`: The value that was expected
    /// - `solution`: String representation of the solution
    #[error("SPARQL solutions error: Not found value for {value} in {solution:?}")]
    NotFoundInSolution { value: String, solution: String },

    /// Error when a term cannot be converted to a subject.
    ///
    /// # Fields
    /// - `term`: The term that is not a subject
    #[error("Expected term {term} to be a subject")]
    NoSubject { term: Term },

    /// Error related to IRI string operations.
    ///
    /// # Fields
    /// - `err`: The underlying IRI string error
    #[error(transparent)]
    IriSError {
        #[from]
        err: IriSError,
    },

    /// Dereferencing an entity IRI (`EndpointStrategy::Dereference`) returned
    /// a non-success HTTP status.
    ///
    /// # Fields
    /// - `uri`: The IRI that was dereferenced
    /// - `status`: The HTTP status returned
    #[error("Dereferencing {uri} returned HTTP status {status}")]
    DereferenceHttpStatus { uri: String, status: String },

    /// The response body from dereferencing an entity IRI couldn't be parsed
    /// as Turtle.
    ///
    /// # Fields
    /// - `uri`: The IRI that was dereferenced
    /// - `error`: Detailed description of the parsing failure
    #[error("Failed to parse Turtle from dereferencing {uri}: {error}")]
    DereferenceParseError { uri: String, error: String },

    /// A `QueryRDF` operation (a raw SPARQL query, or a SPARQL-based node
    /// selector) was attempted against an endpoint using
    /// `EndpointStrategy::Dereference`, which has no SPARQL query service to
    /// send it to — only entity IRIs can be looked up, by dereferencing them.
    ///
    /// # Fields
    /// - `operation`: Which `QueryRDF` operation was attempted (e.g. "SELECT")
    #[error(
        "{operation} is not supported by the 'dereference' endpoint strategy: it only answers \
         lookups by dereferencing entity IRIs, not by running arbitrary SPARQL queries. \
         Use --strategy sparql (the default) for this operation."
    )]
    UnsupportedForDereferenceStrategy { operation: &'static str },
}

/// Converts a reqwest error into an HTTPRequestError.
impl From<reqwest::Error> for OxigraphEndpointError {
    fn from(e: reqwest::Error) -> OxigraphEndpointError {
        OxigraphEndpointError::HTTPRequestError { e }
    }
}

/// Converts a URL parsing error into a URLParseError.
impl From<url::ParseError> for OxigraphEndpointError {
    fn from(e: url::ParseError) -> OxigraphEndpointError {
        OxigraphEndpointError::URLParseError { e }
    }
}

/// Converts a SPARQL results parsing error into an SPAResults error.
impl From<sparesults::QueryResultsParseError> for OxigraphEndpointError {
    fn from(e: sparesults::QueryResultsParseError) -> OxigraphEndpointError {
        OxigraphEndpointError::SPAResults { e }
    }
}
