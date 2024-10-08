// use oxiri::Iri;
// use oxrdf::IriParseError;
use thiserror::Error;
use url::Url;

use crate::IriS;

#[derive(Error, Debug, Clone)]
pub enum IriSError {
    #[error("Error parsing {str} as IRI: {err}")]
    IriParseError { str: String, err: String },

    #[error("Parsing {str} using base: {base} as IRI. Error: {error}")]
    IriParseErrorWithBase {
        str: String,
        base: Box<Url>,
        error: String,
    },

    #[error("Error resolving IRI `{other}` with base IRI `{base}`: {err}")]
    IriResolveError {
        err: Box<String>,
        base: Box<IriS>,
        other: Box<IriS>,
    },

    #[error("Error joining IRI `{current}` with `{str}`: {err}")]
    JoinError {
        err: Box<String>,
        current: Box<IriS>,
        str: Box<String>,
    },
    #[error("Creating reqwest http client: {error}")]
    ReqwestClientCreation { error: String },

    #[error("Parsing Iri {str} as Url. Error: {error}")]
    UrlParseError { str: String, error: String },

    #[error("Http request error: {error}")]
    ReqwestError { error: String },

    #[error("Http request error as String: {error}")]
    ReqwestTextError { error: String },

    #[error("trying to obtain a path from file scheme Url: {url}")]
    ConvertingFileUrlToPath { url: Url },

    #[error("Error reading from file {path} obtained from url {url}. Error: {error}")]
    IOErrorFile {
        path: String,
        url: Box<Url>,
        error: String,
    },
}
