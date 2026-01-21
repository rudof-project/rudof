use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RDFError {
    #[error("Conversion error: {msg}")]
    ConversionError { msg: String },

    #[error("Generic RDF error: {0}")]
    Generic(String),

    #[error("Error obtaining IRI from IriRef: {iri_ref}")]
    IriRefError { iri_ref: String },

    #[error("Error with language tag '{language}' in literal '{literal}': {error}")]
    LanguageTagError {
        literal: String,
        language: String,
        error: String,
    },  

    #[error("Cannot convert literal '{literal}' to {datatype}")]
    LiteralDataTypeParseError { literal: String, datatype: String },
}

impl RDFError {
}
