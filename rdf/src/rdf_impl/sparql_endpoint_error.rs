use crate::rdf_impl::SparqlVars;
use iri_s::error::IriSError;
use oxiri::IriParseError;
use oxrdf::Term;
use thiserror::Error;

/// Represents all possible errors that can occur when interacting with SPARQL endpoints.
#[derive(Error, Debug)]
pub enum SparqlEndpointError {
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
    SPAResults {
        e: sparesults::QueryResultsParseError,
    },

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

    /// Error parsing the response body from the endpoint.
    ///
    /// # Fields
    /// - `body`: The body content that failed to parse
    #[error("Error parsing body: {body}")]
    ParsingBody { body: String },

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
}

/// Converts a reqwest error into an HTTPRequestError.
impl From<reqwest::Error> for SparqlEndpointError {
    fn from(e: reqwest::Error) -> SparqlEndpointError {
        SparqlEndpointError::HTTPRequestError { e }
    }
}

/// Converts a URL parsing error into a URLParseError.
impl From<url::ParseError> for SparqlEndpointError {
    fn from(e: url::ParseError) -> SparqlEndpointError {
        SparqlEndpointError::URLParseError { e }
    }
}

/// Converts a SPARQL results parsing error into an SPAResults error.
impl From<sparesults::QueryResultsParseError> for SparqlEndpointError {
    fn from(e: sparesults::QueryResultsParseError) -> SparqlEndpointError {
        SparqlEndpointError::SPAResults { e }
    }
}
