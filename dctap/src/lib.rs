//! DCTAP Processor
//!
//! This module contains a simple [DCTAP](https://www.dublincore.org/specifications/dctap/) processor
//! 
//! 
//! DCTAP (Dublin Core Tabular Application Profiles) is a simple model that can be used to specify data models
//! 
pub mod dctap;
pub mod dctap_error;
pub mod tap_statement_template;

pub use dctap::*;
pub use crate::tap_statement_template::*;

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
    IriS::new_unchecked($lit)
 }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

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
