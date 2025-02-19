use iri_s::error::GenericIriError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SparqlError {
    #[error("HTTP Request error: {e:?}")]
    HttpRequest { e: reqwest::Error },

    #[error("URL parser error: {e:?}")]
    UrlParse { e: url::ParseError },

    #[error("SPARQL Results parser: {e:?}")]
    SpareResultsParser {
        e: sparesults::QueryResultsParseError,
    },

    #[error("Error parsing the body of the response")]
    ParsingBody,

    #[error("Unknown SPARQL endpoint name {_0}")]
    UnknownEndpointName(String),

    #[error(transparent)]
    Iri(#[from] GenericIriError),
}

impl From<reqwest::Error> for SparqlError {
    fn from(e: reqwest::Error) -> SparqlError {
        SparqlError::HttpRequest { e }
    }
}

impl From<url::ParseError> for SparqlError {
    fn from(e: url::ParseError) -> SparqlError {
        SparqlError::UrlParse { e }
    }
}

impl From<sparesults::QueryResultsParseError> for SparqlError {
    fn from(e: sparesults::QueryResultsParseError) -> SparqlError {
        SparqlError::SpareResultsParser { e }
    }
}
