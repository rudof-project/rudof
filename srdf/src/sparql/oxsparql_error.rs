use iri_s::IriSError;
use oxiri::IriParseError;
use oxrdf::Term;
//use sparesults::QuerySolution;
use thiserror::Error;

use crate::SparqlVars;

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

    #[error(transparent)]
    IriParseError {
        #[from]
        err: IriParseError,
    },

    #[error("Unknown name for endpoint: {name}")]
    UnknownEndpontName { name: String },

    #[error("Error parsing body: {body}")]
    ParsingBody { body: String },

    #[error("SPARQL solutions error: Expected IRI, got {value}")]
    SPARQLSolutionErrorNoIRI { value: Term },

    #[error("SPARQL solutions error: Not found vars {vars} in solution {solution:?}")]
    NotFoundVarsInSolution { vars: SparqlVars, solution: String },

    #[error("SPARQL solutions error: Expected Subject, got {value}")]
    SPARQLSolutionErrorNoSubject { value: Term },

    #[error("SPARQL solutions error: Not found value for {value} in {solution:?}")]
    NotFoundInSolution { value: String, solution: String },

    #[error("Expected term {term} to be a subject")]
    NoSubject { term: Term },

    #[error(transparent)]
    IriSError {
        #[from]
        err: IriSError,
    },
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
