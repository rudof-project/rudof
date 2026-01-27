use crate::{PrefixMap, PrefixMapError};
use iri_s::{IriS, IriSError};
use thiserror::Error;

use crate::IriRefError;

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

/// A trait for dereferencing IRIs with an optional base IRI and a [`PrefixMap`]
pub trait Deref {
    /// Dereferences the IRI using the provided base IRI and [`PrefixMap`]
    ///
    /// Returns a new instance with the dereferenced IRI or a [`DerefError`] if dereferencing fails
    ///
    /// ## Fields:
    /// - `base`: An optional base IRI
    /// - `prefixmap`: An optional [`PrefixMap`]
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized;
}

impl<T: Deref> Deref for Option<T> {
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        self.map(|t| t.deref(base, prefixmap)).transpose()
    }
}

impl<T: Deref> Deref for Box<T> {
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        Ok(Box::new((*self).deref(base, prefixmap)?))
    }
}

impl<T: Deref> Deref for Vec<T> {
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        self.into_iter().map(|t| t.deref(base, prefixmap)).collect()
    }
}
