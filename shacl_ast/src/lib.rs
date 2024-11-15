//! SHACL Abstract Syntax
//!
//! Ths abstract syntax follows the [SHACL spec](https://www.w3.org/TR/shacl/)

#![deny(rust_2018_idioms)]
// The recursion limit is increased because the default one (128) is not enough
// for the big lazy_static declaration in the SHACL vocabulary definition
#![recursion_limit = "256"]
pub mod ast;
// pub mod compiled;
pub mod converter;
pub mod vocab;

pub use ast::*;
pub use converter::*;

/// SHACL Formats supported, which are mostly RDF formats. In the future, we
/// could also support SHACL Compact format.
#[derive(Debug, Clone, Default)]
pub enum ShaclFormat {
    #[default]
    Turtle,
    Internal,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}
