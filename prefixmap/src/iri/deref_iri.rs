use crate::PrefixMap;
use crate::error::DerefError;
use iri_s::IriS;

/// A trait for dereferencing IRIs with an optional base IRI and a [`PrefixMap`]
pub trait DerefIri {
    /// Dereferences the IRI using the provided base IRI and [`PrefixMap`]
    ///
    /// Returns a new instance with the dereferenced IRI or a [`DerefError`] if dereferencing fails
    ///
    /// ## Fields:
    /// - `base`: An optional base IRI
    /// - `prefixmap`: An optional [`PrefixMap`]
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized;
}

impl<T: DerefIri> DerefIri for Option<T> {
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        self.map(|t| t.deref_iri(base, prefixmap)).transpose()
    }
}

impl<T: DerefIri> DerefIri for Box<T> {
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        Ok(Box::new((*self).deref_iri(base, prefixmap)?))
    }
}

impl<T: DerefIri> DerefIri for Vec<T> {
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        self.into_iter().map(|t| t.deref_iri(base, prefixmap)).collect()
    }
}
