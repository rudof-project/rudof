//! SHACL RDF
//! Contains the code that converts SHACL AST / IR to and from RDF

pub(crate) mod error;
mod parser;
mod parsers;
mod test;
mod writer;

pub use parser::ShaclParser;
pub(crate) use parsers::State;
pub use writer::ShaclWriter;
