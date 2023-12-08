use oxiri::IriParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SRDFSparqlError {
    #[error("HTTP Request error: {e:?}")]
    HTTPRequestError { e: reqwest::Error },

    #[error("URL parser error: {e:?}")]
    URLParseError { e: url::ParseError },

    #[error("SPARQL Results parser: {e:?}")]
    SPAResults { e: sparesults::ParseError },

    #[error(transparent)]
    IriParseError {
        #[from]
        err: IriParseError,
    },
}

impl From<reqwest::Error> for SRDFSparqlError {
    fn from(e: reqwest::Error) -> SRDFSparqlError {
        SRDFSparqlError::HTTPRequestError { e: e }
    }
}

impl From<url::ParseError> for SRDFSparqlError {
    fn from(e: url::ParseError) -> SRDFSparqlError {
        SRDFSparqlError::URLParseError { e: e }
    }
}

impl From<sparesults::ParseError> for SRDFSparqlError {
    fn from(e: sparesults::ParseError) -> SRDFSparqlError {
        SRDFSparqlError::SPAResults { e: e }
    }
}
