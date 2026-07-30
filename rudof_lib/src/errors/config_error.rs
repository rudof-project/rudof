use std::io;
use thiserror::Error;

/// Errors related to Rudof configuration loading and parsing.
#[derive(Error, Debug)]
pub enum RudofConfigError {
    /// Error reading configuration from a file path.
    #[error("Error reading config file from path {path}: {error}")]
    ReadError {
        path: String,
        error: String,
    },

    /// Error parsing TOML configuration from a file.
    #[error("Error parsing TOML config from path {path}: {error}")]
    TomlPathError {
        path: String,
        error: String,
    },

    /// Error parsing TOML configuration from a string.
    #[error("Error parsing TOML config from string '{string}': {error}")]
    TomlStringError {
        string: String,
        error: String,
    },
}
