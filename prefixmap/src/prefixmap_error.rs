use crate::PrefixMap;
use iri_s::IriSError;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum PrefixMapError {
    #[error(transparent)]
    IriSError(#[from] IriSError),

    #[error("Prefix '{prefix}' not found in PrefixMap '{prefixmap}'")]
    PrefixNotFound {
        prefix: String,
        prefixmap: PrefixMap,
    },

    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),
}
