use crate::PrefixMap;
use iri_s::IriSError;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum PrefixMapError {
    #[error(transparent)]
    IriSError(#[from] IriSError),

    #[error("Alias '{prefix}' not found in prefix map\nAvailable aliases: [{}]", prefixmap.aliases().cloned().collect::<Vec<_>>().join(", "))]
    PrefixNotFound {
        prefix: String,
        prefixmap: PrefixMap,
    },

    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),

    #[error("IO Error: {error}")]
    IOError { error: String },
}
