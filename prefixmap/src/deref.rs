use crate::{PrefixMap, PrefixMapError};
use iri_s::{IriS, IriSError};
use thiserror::Error;

use crate::IriRefError;

#[derive(Debug, Error, Clone)]
pub enum DerefError {
    #[error(transparent)]
    IriSError(#[from] IriSError),

    #[error("Error obtaining IRI for '{alias}:{local}': {error}")]
    DerefPrefixMapError {
        alias: String,
        local: String,
        error: Box<PrefixMapError>,
    },

    #[error("No prefix map to dereference prefixed name {prefix}{local}")]
    NoPrefixMapPrefixedName { prefix: String, local: String },

    #[error(transparent)]
    UnderefError(#[from] IriRefError),
}

pub trait Deref {
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized;
}

impl <T: Deref> Deref for Option<T> {
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized
    {
        self
            .map(|t| t.deref(base, prefixmap))
            .transpose()
    }
}

impl <T: Deref> Deref for Box<T> {
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized
    {
        Ok(Box::new((*self).deref(base, prefixmap)?))
    }
}

impl <T: Deref> Deref for Vec<T> {
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        self.into_iter().map(|t| t.deref(base, prefixmap)).collect()
    }
}
