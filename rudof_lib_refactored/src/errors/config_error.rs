use thiserror::Error;
use std::io;

/// Errors related to Rudof configuration loading and parsing.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Error reading configuration from a file path.
    #[error("Error reading config file from path {path}: {error}")]
    ReadFromPath {path: String, #[source] error: io::Error},

    /// Error parsing TOML configuration from a file.
    #[error("Error parsing TOML config from path {path}: {error}")]
    TomlParseFromPath {path: String, #[source] error: toml::de::Error},

    /// Error parsing TOML configuration from a string.
    #[error("Error parsing TOML config from string: {error}\nContent:\n{content}")]
    TomlParseFromString {content: String, #[source] error: toml::de::Error},

    /// Generic configuration error with context message.
    #[error("Configuration error: {msg}")]
    Generic { msg: String },
}
