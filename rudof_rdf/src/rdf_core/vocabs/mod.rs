//! Vocabulary constants and accessors for several languages.
//!
//! This module provides compile-time constants and thread-safe singleton accessors for
//! commonly used IRIs from the RDF, XML Schema (XSD) and SHACL vocabularies. These constants
//! represent standard properties and datatypes used throughout RDF processing.

use rudof_iri::IriS;
use std::sync::OnceLock;

mod rdf;
mod rdfs;
mod shacl;
mod shacl_node_expression;
mod shacl_test;
mod shexr;
mod test_manifest;
mod xsd;

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

    fn base_iri() -> &'static IriS {
        static IRI: OnceLock<IriS> = OnceLock::new();
        IRI.get_or_init(|| IriS::new_unchecked(Self::BASE))
    }
}
