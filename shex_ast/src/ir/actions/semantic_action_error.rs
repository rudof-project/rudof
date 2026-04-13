use thiserror::*;

#[derive(Error, Debug)]
pub enum SemanticActionError {
    #[error("Expected non-empty string parameter but no value passed")]
    ExpectedParameterButEmpty,

    #[error("Invalid Test semact parameter: {parameter:?}")]
    InvalidTestParameter { parameter: String },

    #[error("Invalid IRI parameter: {details}")]
    InvalidParameter { details: String },

    #[error("No object provided in context for Test semact: {details}")]
    NoObjectInContext { details: String },

    #[error("Unresolved variable {variable:?} in Test semact: no binding provided")]
    UnresolvedVariable { variable: String },

    #[error("Test semact fail: {message}")]
    FailAction { message: String },

    #[error("No extension registered for IRI {iri}")]
    UnknownExtension { iri: String },
}
