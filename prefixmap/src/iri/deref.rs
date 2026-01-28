use crate::error::DerefError;
use crate::PrefixMap;
use iri_s::IriS;

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
