use crate::ShaclError;
use srdf::RDFParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaclParserError {
    #[error("RDF parse error: {err}")]
    RDFParseError {
        #[from]
        err: RDFParseError,
    },

    #[error("Expected RDFNode parsing node shape, found: {term}")]
    ExpectedRDFNodeNodeShape { term: String },

    #[error("Expected Value of `sh:or` to be a subject, found: {term}")]
    OrValueNoSubject { term: String },

    #[error("Expected Value of `sh:and` to be a subject, found: {term}")]
    AndValueNoSubject { term: String },

    #[error("Expected Value of `sh:xone` to be a subject, found: {term}")]
    XOneValueNoSubject { term: String },

    #[error("Expected NodeKind, found: {term}")]
    ExpectedNodeKind { term: String },

    #[error("Unknown NodeKind, found: {term}")]
    UnknownNodeKind { term: String },

    #[error("SHACL error: {err}")]
    ShaclError {
        #[from]
        err: ShaclError,
    },

    #[error("Custom error: {msg}")]
    Custom { msg: String },
}
