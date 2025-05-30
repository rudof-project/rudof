use crate::PrefixMap;
use iri_s::IriSError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Clone, Serialize)]
pub enum PrefixMapError {
    #[error(transparent)]
    IriSError(#[from] IriSError),

    #[error("Prefix '{prefix}' not found in PrefixMap '{prefixmap}'")]
    PrefixNotFound {
        prefix: String,
        prefixmap: PrefixMap,
    },

    #[error("Format error: {error}")]
    FormatError { error: String },
}
