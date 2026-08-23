//! Vocabulary constants and accessors for several languages.
//!
//! This module provides compile-time constants and thread-safe singleton accessors for
//! commonly used IRIs from the RDF, XML Schema (XSD) and SHACL vocabularies. These constants
//! represent standard properties and datatypes used throughout RDF processing.

use rudof_iri::IriS;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

mod owl;
mod rdf;
mod rdfs;
mod shacl;
mod shacl_node_expression;
mod shacl_test;
mod shexr;
mod test_manifest;
mod xsd;

pub use owl::OwlVocab;
pub use rdf::RdfVocab;
pub use rdfs::RdfsVocab;
pub use shacl::ShaclVocab;
pub use shacl_node_expression::ShaclNodeExprVocab;
pub use shacl_test::ShaclTestVocab;
pub use shexr::ShexRVocab;
pub use test_manifest::TestManifestVocab;
pub use xsd::XsdVocab;

#[macro_export]
macro_rules! vocab_term {
    ($voc:ident, $name:ident, $suffix:literal) => {
        impl $voc {
            pub const $name: &'static str = const_format::concatcp!($voc::BASE, $suffix);

            paste::paste! {
                pub fn [<$name:lower _ref>]() -> &'static rudof_iri::IriS {
                    static IRI: std::sync::OnceLock<rudof_iri::IriS> = std::sync::OnceLock::new();
                    IRI.get_or_init(|| rudof_iri::IriS::new_unchecked(Self::$name))
                }

                #[inline]
                pub fn [<$name:lower>]() -> rudof_iri::IriS {
                    Self::[<$name:lower _ref>]().clone()
                }
            }
        }
    };
}

pub trait RdfVocabulary {
    const BASE: &'static str;

    /// Returns the base IRI of this vocabulary.
    ///
    /// This is a default trait method, so its body is shared source across
    /// every implementor; a `static` declared directly in it is **not**
    /// distinct per `Self` (unlike a `static` in a macro-expanded inherent
    /// method, where each expansion is separate source). Caching keyed only
    /// by `TypeId`/monomorphization here previously collapsed to a single
    /// shared cell, so whichever vocabulary's `base_iri()` ran first "won"
    /// for every other vocabulary in the process. Keying explicitly by
    /// `Self::BASE` (unique per vocabulary) avoids that.
    fn base_iri() -> &'static IriS {
        static CACHE: OnceLock<Mutex<HashMap<&'static str, &'static IriS>>> = OnceLock::new();
        let mut cache = CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
        *cache
            .entry(Self::BASE)
            .or_insert_with(|| Box::leak(Box::new(IriS::new_unchecked(Self::BASE))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regardless of which vocabulary's `base_iri()` runs first in the
    /// process, every vocabulary must keep returning its own base IRI.
    #[test]
    fn base_iri_is_distinct_per_vocabulary() {
        assert_eq!(RdfVocab::base_iri().as_str(), RdfVocab::BASE);
        assert_eq!(ShexRVocab::base_iri().as_str(), ShexRVocab::BASE);
        assert_eq!(XsdVocab::base_iri().as_str(), XsdVocab::BASE);
        assert_eq!(RdfsVocab::base_iri().as_str(), RdfsVocab::BASE);
        assert_ne!(RdfVocab::base_iri().as_str(), ShexRVocab::base_iri().as_str());
    }
}
