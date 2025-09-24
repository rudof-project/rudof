//! SHACL Abstract Syntax
//!
//! Ths abstract syntax follows the [SHACL spec](https://www.w3.org/TR/shacl/)
//!

#![deny(rust_2018_idioms)]
pub mod ast;
pub mod shacl_vocab;
pub use ast::*;
pub use shacl_vocab::*;

/// SHACL Formats supported. Mostly RDF formats
/// In the future, we could also support SHACL Compact format
#[derive(Debug, Clone, Default)]
pub enum ShaclFormat {
    Internal,
    #[default]
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl ShaclFormat {
    /// Returns the MIME type for the SHACL format
    pub fn mime_type(&self) -> &str {
        match self {
            ShaclFormat::Internal => "application/shacl+json",
            ShaclFormat::Turtle => "text/turtle",
            ShaclFormat::NTriples => "application/n-triples",
            ShaclFormat::RDFXML => "application/rdf+xml",
            ShaclFormat::TriG => "application/trig",
            ShaclFormat::N3 => "text/n3",
            ShaclFormat::NQuads => "application/n-quads",
        }
    }
}
