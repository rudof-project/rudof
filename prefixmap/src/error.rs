use iri_s::error::GenericIriError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DerefError {
    #[error(transparent)]
    PrefixMap(#[from] PrefixMapError),
    #[error("No prefix map to dereference prefixed name {prefix}:{local}")]
    NoPrefixMapPrefixedName { prefix: String, local: String },
    #[error("Cannot obtain IRI from prefixed name IriRef {}:{}", _0, _1)]
    Underef(String, String),
    #[error(transparent)]
    Iri(#[from] GenericIriError),
}

#[derive(Debug, Error, Clone)]
pub enum PrefixMapError {
    #[error("Prefix '{prefix}' not found in PrefixMap '{prefixmap}'")]
    PrefixNotFound { prefix: String, prefixmap: String },
    #[error(transparent)]
    Format(#[from] IriFromStrError),
    #[error(transparent)]
    Iri(#[from] GenericIriError),
}

#[derive(Error, Debug, Clone)]
#[error("Error parsing the {} IRI from a String", ._0)]
pub struct IriFromStrError(pub String);
