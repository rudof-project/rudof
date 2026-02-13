use crate::iri::IriS;
use serde::Serialize;
use thiserror::Error;

/// Errors that can occur when working with [`IriS`]
#[derive(Error, Debug, Clone, Serialize)]
pub enum IriSError {
    /// Error converting a [`std::path::Path`] into an IRI
    #[error("Error converting path {path} to IRI: {error}")]
    ConvertingPathToIri { path: String, error: String },

    /// Error parsing an [`String`] into an IRI
    #[error("Error parsing {str} as IRI: {error}")]
    IriParseError { str: String, error: String },

    /// Error parsing an [`String`] into an IRI using a base IRI
    #[error("Parsing {str} using base: {base} as IRI. Error: {error}")]
    IriParseErrorWithBase { str: String, base: String, error: String },

    /// Error resolving an IRI against a base IRI
    #[error("Error resolving IRI `{other}` with base IRI `{base}`: {error}")]
    IriResolveError {
        error: String,
        base: Box<IriS>,
        other: String,
    },

    /// Error joining an IRI with a [`String`]
    #[error("Error joining IRI `{current}` with `{str}`: {error}")]
    JoinError {
        error: String,
        current: Box<IriS>,
        str: String,
    },

    /// Error creating a [`reqwest`] HTTP client
    #[error("Creating reqwest http client: {error}")]
    ReqwestClientCreation { error: String },

    /// Error parsing an IRI as a [`url::Url`]
    #[error("Parsing Iri {str} as Url. Error: {error}")]
    UrlParseError { str: String, error: String },

    /// Error performing an HTTP request with [`reqwest`]
    #[error("Http request error: {error}")]
    ReqwestError { error: String },

    /// Error performing an HTTP request with [`reqwest`] and obtaining the response as text
    #[error("Http request error as String: {error}")]
    ReqwestTextError { error: String },

    /// Error converting a file scheme [`url::Url`] into a [`std::path::Path`]
    #[error("trying to obtain a path from file scheme Url: {url}")]
    ConvertingFileUrlToPath { url: String },

    /// Error reading from a file obtained from a [`url::Url`]
    #[error("Error reading from file {path} obtained from url {url}. Error: {error}")]
    IOErrorFile { path: String, url: String, error: String },
}
