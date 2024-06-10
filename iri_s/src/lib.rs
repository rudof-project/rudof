//! IRI simple wrapper
//!
//! This module contains a simple wrapper to work with IRIs
//! The main goal is that we can use a simple interface to work with IRIs
//! which could be adapted to different implementations in the future if needed.
//!
//! The library provides the macro [`iri`] to create IRIs from strings.
//!
// pub mod iri;
pub mod iris;
pub mod iris_error;

// pub use iri::*;
pub use iris::*;
pub use iris_error::*;

/// ```
///
/// #[macro_use]
/// use iri_s::{IriS, iri};
///
/// let iri = iri!("http://example.org/");
///
/// assert_eq!(iri.as_str(), "http://example.org/");
/// ```
///
/// At this moment the implementation leverages on [`oxrdf::NamedNode`](https://docs.rs/oxrdf/latest/oxrdf/struct.NamedNode.html)
///
///
///
/// Example
///
/// ```
/// use iri_s::IriS;
/// use std::str::FromStr;
///
/// let iri = IriS::from_str("http://example.org/").unwrap();
///
/// assert_eq!(iri.as_str(), "http://example.org/")
/// ```
///
#[macro_export]
macro_rules! iri {
    (
   $lit: tt
 ) => {
        $crate::IriS::new_unchecked($lit)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn iri_s_test() {
        let iri1: IriS = IriS::from_str("http://example.org/iri").unwrap();
        let iri2 = IriS::from_str("http://example.org/iri").unwrap();
        assert_eq!(iri1, iri2);
    }

    #[test]
    fn test_macro() {
        let iri = iri!("http://example.org/");
        assert_eq!(iri.as_str(), "http://example.org/")
    }
}
