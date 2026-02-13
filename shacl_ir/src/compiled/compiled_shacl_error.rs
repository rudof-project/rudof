use shacl_ast::ShaclError;
use shacl_rdf::shacl_parser_error::ShaclParserError;
use rdf::rdf_core::{utils::RDFRegexError, term::Object};
use rdf::rdf_impl::InMemoryGraphError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompiledShaclError {
    #[error("Duplicate shape: {shape}")]
    DuplicateShape { shape: Box<Object> },

    #[error(
        "Invalid path for property shape with reifier shape {shape_id}, the path must be a single predicate, but got: {path}"
    )]
    InvalidReifierShapePath {
        shape_id: Box<Object>,
        path: String,
    },
    #[error("Conversion from IriRef {iri_ref } failed: {err}")]
    IriRefConversion { iri_ref: String, err: String },

    #[error("Shape not found {shape} ")]
    ShapeNotFound { shape: Box<Object> },

    #[error("Could not convert to Literal: {node}")]
    LiteralConversion { node: Box<Object> },

    #[error("RDF error: {err}")]
    RdfGraphError { err: Box<InMemoryGraphError> },

    #[error("Error parsing SHACL: {err}")]
    ShaclParserError { err: Box<ShaclParserError> },

    #[error("ShaclError: {source}")]
    ShaclError { source: ShaclError },

    #[error("Invalid regex pattern: {pattern} with flags: {}: {error}", flags.as_deref().unwrap_or("None"))]
    InvalidRegex {
        pattern: String,
        flags: Option<String>,
        error: Box<RDFRegexError>,
    },
}
