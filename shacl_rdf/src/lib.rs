//! SHACL RDF
//!
//! Converts between SHACl AST and RDF
//!
#![deny(rust_2018_idioms)]

pub mod rdf_to_shacl;
pub mod shacl_to_rdf;

pub use rdf_to_shacl::*;
pub use shacl_to_rdf::*;