use api::Iri;
use thiserror::Error;

use crate::PrefixMap;
use crate::PrefixMapError;

#[derive(Debug, Error)]
pub enum DerefError {
    #[error(transparent)]
    PrefixMap(#[from] PrefixMapError),
    #[error("No prefix map to dereference prefixed name {prefix}:{local}")]
    NoPrefixMapPrefixedName { prefix: String, local: String },
    #[error("Cannot obtain IRI from prefixed name IriRef {}:{}", _0, _1)]
    Underef(String, String),
}

pub trait Deref<I: Iri + Clone>
where
    Self: Sized,
{
    fn deref(&self, base: &Option<I>, prefixmap: &Option<PrefixMap<I>>)
        -> Result<Self, DerefError>;

    fn deref_opt(
        maybe: &Option<Self>,
        base: &Option<I>,
        prefixmap: &Option<PrefixMap<I>>,
    ) -> Result<Option<Self>, DerefError> {
        maybe.as_ref().map(|t| t.deref(base, prefixmap)).transpose()
    }

    fn deref_opt_box(
        maybe: &Option<Box<Self>>,
        base: &Option<I>,
        prefixmap: &Option<PrefixMap<I>>,
    ) -> Result<Option<Box<Self>>, DerefError> {
        maybe
            .as_ref()
            .map(|t| t.deref(base, prefixmap))
            .transpose()
            .map(|t| t.map(|t| Box::new(t)))
    }

    fn deref_vec(
        ts: &[Self],
        base: &Option<I>,
        prefixmap: &Option<PrefixMap<I>>,
    ) -> Result<Vec<Self>, DerefError> {
        ts.iter().map(|t| t.deref(base, prefixmap)).collect()
    }

    fn deref_slice_box(
        ts: &[Box<Self>],
        base: &Option<I>,
        prefixmap: &Option<PrefixMap<I>>,
    ) -> Result<Vec<Self>, DerefError> {
        ts.iter().map(|t| t.deref(base, prefixmap)).collect()
    }

    fn deref_opt_vec(
        maybe_ts: &Option<Vec<Self>>,
        base: &Option<I>,
        prefixmap: &Option<PrefixMap<I>>,
    ) -> Result<Option<Vec<Self>>, DerefError> {
        maybe_ts
            .as_ref()
            .map(|ts| ts.iter().map(|t| t.deref(base, prefixmap)).collect())
            .transpose()
    }
}
