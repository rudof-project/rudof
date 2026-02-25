//! Vocabulary constants and accessors for several languages.
//!
//! This module provides compile-time constants and thread-safe singleton accessors for
//! commonly used IRIs from the RDF, XML Schema (XSD) and SHACL vocabularies. These constants
//! represent standard properties and datatypes used throughout RDF processing.

use iri_s::IriS;
use std::sync::OnceLock;

mod rdf;
mod rdfs;
mod shacl;
mod shacl_node_expression;
mod xsd;

pub use rdf::RdfVocab;
pub use rdfs::RdfsVocab;
pub use shacl::ShaclVocab;
pub use shacl_node_expression::ShaclNodeExprVocab;
pub use xsd::XsdVocab;

#[macro_export]
macro_rules! vocab_term {
    ($voc:ident, $name:ident, $suffix:literal) => {
        impl $voc {
            pub const $name: &'static str = const_format::concatcp!($voc::BASE, $suffix);

            paste::paste! {
                pub fn [<$name:lower>]() -> &'static iri_s::IriS {
                    static IRI: std::sync::OnceLock<iri_s::IriS> = std::sync::OnceLock::new();
                    IRI.get_or_init(|| iri_s::IriS::new_unchecked(Self::$name))
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
