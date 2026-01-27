use crate::PrefixMap;
use iri_s::IriSError;
use thiserror::Error;

/// Errors that can occur when working with [`PrefixMap`]
#[derive(Debug, Error, Clone)]
pub enum PrefixMapError {
    /// Error originating from the [`iri_s`] crate
    #[error(transparent)]
    IriSError(#[from] IriSError),

    /// Error indicating that a requested prefix alias was not found in the [`PrefixMap`]
    #[error("Alias '{prefix}' not found in prefix map\nAvailable aliases: [{}]", prefixmap.aliases().cloned().collect::<Vec<_>>().join(", "))]
    PrefixNotFound {
        prefix: String,
        prefixmap: PrefixMap,
    },

    /// Error indicating a formatting issue
    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),

    /// Error indicating an IO issue
    #[error("IO Error: {error}")]
    IOError { error: String },

    /// Error indicating that an alias already exists in the [`PrefixMap`]
    #[error("Alias '{prefix}' already exists in prefix map with value '{value}'")]
    AliasAlreadyExists { prefix: String, value: String },
}
