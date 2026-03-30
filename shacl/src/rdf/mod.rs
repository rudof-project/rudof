//! SHACL RDF
//! Contains the code that converts SHACL AST / IR to and from RDF

mod parsers;
mod parser;
mod writer;
pub mod error;
mod test;

pub use parser::ShaclParser;
pub(crate) use parsers::State;
pub use writer::ShaclWriter;
