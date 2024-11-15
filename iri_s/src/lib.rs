//! Simple wrapper over IRIs.
//!
//! This module contains a simple wrapper to work with IRIs. The main goal is
//! that we can use a simple interface to work with IRIs which could be adapted
//! to different implementations in the future if needed.
//!
//! The library provides the macro [`iri`] to create IRIs from strings.
pub use iris::*;

pub mod error;
pub mod iris;

/// The `iri` macro can be used to easily create IRIs in your projects. By
/// providing an string, one can obtain the desired IRI obtained from it. This
/// is the expected mechanism for handling IRIs.
///
/// ```
/// use iri_s::IriS;
/// use std::str::FromStr;
///
/// let iri = IriS::from_str("http://example.org/").unwrap();
/// assert_eq!(iri.as_str(), "http://example.org/")
/// ```
#[macro_export]
macro_rules! iri {
    (
   $lit: tt
 ) => {
        $crate::GenericIri::new_unchecked($lit.to_string())
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn iri_s_test() {
        let iri1 = GenericIri::<String>::from_str("http://example.org/iri").unwrap();
        let iri2 = GenericIri::<String>::from_str("http://example.org/iri").unwrap();
        assert_eq!(iri1, iri2);
    }

    #[test]
    fn test_macro() {
        let iri = iri!("http://example.org/");
        assert_eq!(iri.as_str(), "http://example.org/")
    }
}
