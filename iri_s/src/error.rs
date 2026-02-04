use crate::iri::IriS;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize)]
pub enum IriSError {
    #[error("Error converting path {path} to IRI: {error}")]
    ConvertingPathToIri { path: String, error: String },

    #[error("Error parsing {str} as IRI: {error}")]
    IriParseError { str: String, error: String },

    #[error("Parsing {str} using base: {base} as IRI. Error: {error}")]
    IriParseErrorWithBase {
        str: String,
        base: String,
        error: String,
    },

    #[error("Error resolving IRI `{other}` with base IRI `{base}`: {error}")]
    IriResolveError {
        error: String,
        base: Box<IriS>,
        other: String,
    },

    #[error("Error joining IRI `{current}` with `{str}`: {error}")]
    JoinError {
        error: String,
        current: Box<IriS>,
        str: String,
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
    ConvertingFileUrlToPath { url: String },

    #[error("Error reading from file {path} obtained from url {url}. Error: {error}")]
    IOErrorFile {
        path: String,
        url: String,
        error: String,
    },

    #[cfg(target_arch = "wasm32")]
    #[error("Error getting IRI from file path in WASM")]
    WASMFilePath,
}
