use crate::PrefixMap;
use iri_s::IriSError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Clone, Serialize)]
pub enum PrefixMapError {
    #[error(transparent)]
    IriSError(#[from] IriSError),

    #[error("Alias '{prefix}' not found in prefix map\nAvailable aliases: [{}]", prefixmap.aliases().cloned().collect::<Vec<_>>().join(", "))]
    PrefixNotFound {
        prefix: String,
        prefixmap: PrefixMap,
    },

    #[error("Format error: {error}")]
    FormatError { error: String },
}
