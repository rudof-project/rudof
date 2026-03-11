use thiserror::Error;
use std::io;
use std::path::PathBuf;

/// Errors that can occur when working with input specifications.
#[derive(Error, Debug)]
pub enum InputSpecError {
    /// Failed to resolve a path to an absolute path.
    #[error("Cannot get absolute path for '{path}': {error}")]
    AbsolutePathError { path: String, error: io::Error },

    /// Failed to convert a file path to a URL.
    #[error("Cannot convert file path to URL: {path:?}")]
    FromFilePath { path: PathBuf },

    /// Failed to open a file for reading.
    #[error("Error opening path {path:?} for reading ({msg}): {err}")]
    OpenPathError {
        msg: String,
        path: PathBuf,
        err: io::Error,
    },

    /// Invalid HTTP Accept header value.
    #[error("Invalid Accept header value '{str}' in context '{context}': {error}")]
    AcceptValue {
        context: String,
        str: String,
        error: String,
    },

    /// Invalid User-Agent header value.
    #[error("Invalid User-Agent header value: {error}")]
    UserAgentValue { error: String },

    /// Failed to build HTTP client.
    #[error("Failed to build HTTP client: {error}")]
    ClientBuilderError { error: String },

    /// Failed to fetch data from URL.
    #[error("Error dereferencing URL {url}: {error}")]
    UrlDerefError { url: url::Url, error: String },

    /// Failed to guess base IRI from file path.
    #[error("Cannot guess base IRI from path: {path:?}")]
    GuessBaseFromPath { path: PathBuf },

    /// Failed to parse a string as a file path.
    #[error("Error parsing path from string '{str}': {error}")]
    ParsingPathError { str: String, error: String },

    /// Failed to parse a string as a URL.
    #[error("Error parsing URL from string '{str}': {error}")]
    UrlParseError { str: String, error: String },
}