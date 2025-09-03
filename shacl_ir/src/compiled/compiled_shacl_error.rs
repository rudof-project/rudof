use shacl_ast::ShaclError;
use srdf::RDFNode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompiledShaclError {
    #[error("Conversion from IriRef failed")]
    IriRefConversion,

    #[error("Shape not found {shape} ")]
    ShapeNotFound { shape: RDFNode },

    #[error("Could not convert to Literal")]
    LiteralConversion,

    #[error("RDFError: {0}")]
    RdfGraphError(#[from] srdf::SRDFGraphError),

    #[error("ShaclParserError: {0}")]
    ShaclParserError(#[from] shacl_rdf::rdf_to_shacl::shacl_parser_error::ShaclParserError),

    #[error(transparent)]
    ShaclError(#[from] ShaclError),

    #[error("Invalid regex pattern: {pattern} with flags: {}: {error}", flags.as_deref().unwrap_or("None"))]
    InvalidRegex {
        pattern: String,
        flags: Option<String>,
        error: srdf::regex::SRegexError,
    },
}
