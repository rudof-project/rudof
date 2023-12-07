use iri_s::IriSError;
use thiserror::Error;
use crate::PrefixMap;

#[derive(Debug, Error)]
pub enum PrefixMapError {

    #[error(transparent)]
    IriSError(#[from] IriSError),

    #[error("Prefix '{prefix}' not found in PrefixMap '{prefixmap}'")]
    PrefixNotFound {
        prefix: String,
        prefixmap: PrefixMap
    }

}