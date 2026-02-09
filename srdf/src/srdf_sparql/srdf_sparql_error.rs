use iri_s::error::IriSError;
use oxiri::IriParseError;
use oxrdf::Term;
//use sparesults::QuerySolution;
use thiserror::Error;

use crate::SparqlVars;

#[derive(Error, Debug)]
pub enum SRDFSparqlError {
    #[error("SPARQL parse error: {error}, query:\n{query_str}")]
    SPARQLParseError { query_str: String, error: String },

    #[error("Unsupported format for CONSTRUCT query: {format:?}")]
    UnsupportedConstructFormat { format: String },

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
    UnknownEndpointName { name: String },

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

    #[cfg(target_family = "wasm")]
    #[error("Function not supported for WASM: {fn_name}")]
    WASMError {
        fn_name: String,
    }
}

impl From<reqwest::Error> for SRDFSparqlError {
    fn from(e: reqwest::Error) -> SRDFSparqlError {
        SRDFSparqlError::HTTPRequestError { e }
    }
}

impl From<url::ParseError> for SRDFSparqlError {
    fn from(e: url::ParseError) -> SRDFSparqlError {
        SRDFSparqlError::URLParseError { e }
    }
}

impl From<sparesults::QueryResultsParseError> for SRDFSparqlError {
    fn from(e: sparesults::QueryResultsParseError) -> SRDFSparqlError {
        SRDFSparqlError::SPAResults { e }
    }
}
