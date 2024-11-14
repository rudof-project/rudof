use iri_s::IriS;

use crate::DerefError;
use crate::PrefixMap;

pub trait Deref: Sized {
    fn deref(&self, base: &Option<IriS>, prefixmap: &Option<PrefixMap>)
        -> Result<Self, DerefError>;

    fn deref_opt(
        maybe: &Option<Self>,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Option<Self>, DerefError> {
        maybe.as_ref().map(|t| t.deref(base, prefixmap)).transpose()
    }

    fn deref_opt_box(
        maybe: &Option<Box<Self>>,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Option<Box<Self>>, DerefError> {
        maybe
            .as_ref()
            .map(|t| t.deref(base, prefixmap))
            .transpose()
            .map(|t| t.map(|t| Box::new(t)))
    }

    fn deref_vec(
        ts: &[Self],
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Vec<Self>, DerefError> {
        ts.iter().map(|t| t.deref(base, prefixmap)).collect()
    }

    fn deref_slice_box(
        ts: &[Box<Self>],
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Vec<Self>, DerefError> {
        ts.iter().map(|t| t.deref(base, prefixmap)).collect()
    }

    fn deref_opt_vec(
        maybe_ts: &Option<Vec<Self>>,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Option<Vec<Self>>, DerefError> {
        maybe_ts
            .as_ref()
            .map(|ts| ts.iter().map(|t| t.deref(base, prefixmap)).collect())
            .transpose()
    }
}
