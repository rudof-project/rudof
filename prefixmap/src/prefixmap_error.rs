use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum PrefixMapError {
    #[error("Prefix '{prefix}' not found in PrefixMap '{prefixmap}'")]
    PrefixNotFound { prefix: String, prefixmap: String },
    #[error("Format error: {error}")]
    FormatError { error: String },
}
