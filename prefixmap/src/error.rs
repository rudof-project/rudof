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


/// Error returned when trying to get an IRI from a prefixed name [`IriRef`]
#[derive(Debug, Error, Clone)]
#[error("Cannot obtain IRI from prefixed name IriRef {prefix}:{local}")]
pub struct IriRefError {
    pub prefix: String,
    pub local: String,
}


/// Represents all the possible errors that can occur when dereferencing IRIs
#[derive(Debug, Error, Clone)]
pub enum DerefError {
    /// An error originating from the [`iri_s`] crate
    #[error(transparent)]
    IriSError(#[from] IriSError),

    /// An error originating when obtaining an IRI from a [`PrefixMap`]
    ///
    /// ## Fields
    /// - `error`: The underlying [`PrefixMapError`]
    #[error("Error obtaining IRI for '{alias}:{local}': {error}")]
    DerefPrefixMapError {
        alias: String,
        local: String,
        error: Box<PrefixMapError>,
    },

    /// An error occured when trying to dereference a prefixed name without a prefix map
    #[error("No prefix map to dereference prefixed name {prefix}{local}")]
    NoPrefixMapPrefixedName { prefix: String, local: String },

    /// An error occured when trying to obtain an IRI from an [`IriRef`]
    #[error(transparent)]
    UnderefError(#[from] IriRefError),
}