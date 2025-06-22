use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompiledShaclError {
    #[error("Conversion from IriRef failed")]
    IriRefConversion,

    #[error("Could not found the shape that it was been searched")]
    ShapeNotFound,

    #[error("Could not convert to Literal")]
    LiteralConversion,

    #[error("RDFError: {0}")]
    RdfGraphError(#[from] srdf::SRDFGraphError),

    #[error("ShaclParserError: {0}")]
    ShaclParserError(#[from] shacl_rdf::rdf_to_shacl::shacl_parser_error::ShaclParserError),
}
