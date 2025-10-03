use shacl_ast::ShaclError;
use shacl_rdf::shacl_parser_error::ShaclParserError;
use srdf::RDFNode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompiledShaclError {
    #[error("Conversion from IriRef {iri_ref } failed: {err}")]
    IriRefConversion { iri_ref: String, err: String },

    #[error("Shape not found {shape} ")]
    ShapeNotFound { shape: Box<RDFNode> },

    #[error("Could not convert to Literal: {node}")]
    LiteralConversion { node: Box<RDFNode> },

    #[error("RDF error: {err}")]
    RdfGraphError { err: Box<srdf::SRDFGraphError> },

    #[error("Error parsing SHACL: {err}")]
    ShaclParserError { err: Box<ShaclParserError> },

    #[error("ShaclError: {source}")]
    ShaclError { source: ShaclError },

    #[error("Invalid regex pattern: {pattern} with flags: {}: {error}", flags.as_deref().unwrap_or("None"))]
    InvalidRegex {
        pattern: String,
        flags: Option<String>,
        error: Box<srdf::regex::SRegexError>,
    },
}
