use crate::{PrefixMap, PrefixMapError};
use iri_s::{IriS, IriSError};
use thiserror::Error;

use crate::Underef;

#[derive(Debug, Error)]
pub enum DerefError {
    #[error(transparent)]
    IriSError(#[from] IriSError),

    #[error(transparent)]
    PrefixMapError(#[from] PrefixMapError),

    #[error("No prefix map to dereference prefixed name {prefix}{local}")]
    NoPrefixMapPrefixedName { prefix: String, local: String },

    #[error(transparent)]
    UnderefError(#[from] Underef),
}

pub trait Deref {
    fn deref(&self, base: &Option<IriS>, prefixmap: &Option<PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized;

    fn deref_opt<T>(
        maybe: &Option<T>,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Option<T>, DerefError>
    where
        T: Deref,
    {
        maybe.as_ref().map(|t| t.deref(base, prefixmap)).transpose()
    }

    fn deref_opt_box<T>(
        maybe: &Option<Box<T>>,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Option<Box<T>>, DerefError>
    where
        T: Deref,
    {
        maybe
            .as_ref()
            .map(|t| t.deref(base, prefixmap))
            .transpose()
            .map(|t| t.map(|t| Box::new(t)))
    }

    fn deref_vec<T>(
        ts: &[T],
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Vec<T>, DerefError>
    where
        T: Deref,
    {
        ts.iter().map(|t| t.deref(base, prefixmap)).collect()
    }

    fn deref_vec_box<T>(
        ts: &[Box<T>],
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Vec<T>, DerefError>
    where
        T: Deref,
    {
        ts.iter().map(|t| t.deref(base, prefixmap)).collect()
    }

    fn deref_opt_vec<T>(
        maybe_ts: &Option<Vec<T>>,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Option<Vec<T>>, DerefError>
    where
        T: Deref,
    {
        maybe_ts
            .as_ref()
            .map(|ts| ts.iter().map(|t| t.deref(base, prefixmap)).collect())
            .transpose()
    }
}
