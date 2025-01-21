use iri_s::error::GenericIriError;
use oxrdf::Term;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SparqlError {
    #[error("HTTP Request error: {e:?}")]
    HTTPRequestError { e: reqwest::Error },

    #[error("URL parser error: {e:?}")]
    URLParseError { e: url::ParseError },

    #[error("SPARQL Results parser: {e:?}")]
    SPAResults {
        e: sparesults::QueryResultsParseError,
    },

    #[error("Unknown name for endpoint: {name}")]
    UnknownEndpointName { name: String },

    #[error("Error parsing the body of the SPARQL response")]
    ParsingBody,

    #[error("SPARQL solutions error: Expected IRI, got {value}")]
    SPARQLSolutionErrorNoIRI { value: Term },

    #[error("SPARQL solutions error: Expected Subject, got {value}")]
    SPARQLSolutionErrorNoSubject { value: Term },

    #[error("SPARQL solutions error: Not found value for {value} in {solution:?}")]
    NotFoundInSolution { value: String, solution: String },

    #[error("Expected term {term} to be a subject")]
    NoSubject { term: Term },

    #[error(transparent)]
    SimpleIri(#[from] GenericIriError),

    #[error("Could not convert the term: {term} to a {term_type}")]
    ConversionError { term: String, term_type: String },
}

impl From<reqwest::Error> for SparqlError {
    fn from(e: reqwest::Error) -> SparqlError {
        SparqlError::HTTPRequestError { e }
    }
}

impl From<url::ParseError> for SparqlError {
    fn from(e: url::ParseError) -> SparqlError {
        SparqlError::URLParseError { e }
    }
}

impl From<sparesults::QueryResultsParseError> for SparqlError {
    fn from(e: sparesults::QueryResultsParseError) -> SparqlError {
        SparqlError::SPAResults { e }
    }
}
