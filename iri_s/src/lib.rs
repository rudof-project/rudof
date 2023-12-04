pub mod iri;
pub mod iris;
pub mod iris_error;


//! IRI simple wrapper
//! 
//! This module contains a simple wrapper to work with IRIs
//! The main goal is that we can use a simple interface to work with IRIs 
//! which could be adapted to different implementations in the future if needed.
//! 
//! At this moment the implementation leverages on [`oxrdf::NamedNode`](https://docs.rs/oxrdf/latest/oxrdf/struct.NamedNode.html)

// pub use iri::*;
pub use iris::*;
pub use iris_error::*;

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
}
