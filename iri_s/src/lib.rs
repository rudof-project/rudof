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

/// This macro creates a static variable that is initialized once and can be accessed globally.
#[macro_export]
macro_rules! static_once {
    ($name:ident, $type:ty, $init:expr) => {
        pub fn $name() -> &'static $type {
            static ONCE: std::sync::OnceLock<$type> = std::sync::OnceLock::new();
            ONCE.get_or_init(|| $init)
        }
    };
}

/// This macro creates a static variable that is initialized once and can be accessed globally.
#[macro_export]
macro_rules! iri_once {
    ($name:ident, $str:expr) => {
        pub fn $name() -> & 'static IriS {
            static ONCE: std::sync::OnceLock<IriS> = std::sync::OnceLock::new();
            ONCE.get_or_init(|| IriS::new_unchecked($str))
        }
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
    fn test_macro_iri() {
        let iri = iri!("http://example.org/");
        assert_eq!(iri.as_str(), "http://example.org/")
    }

    #[test]
    fn test_macro_static_once() {
        static_once!(example, IriS, IriS::new_unchecked("http://example.org/"));
        let iri = example();
        assert_eq!(iri.as_str(), "http://example.org/")
    }

    #[test]
    fn test_macro_iri_lazy() {
        iri_once!(example, "http://example.org/");
        let iri = example();
        assert_eq!(iri.as_str(), "http://example.org/")
    }


}
