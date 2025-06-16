use shacl_ast::ShaclError;
use srdf::RDFParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaclParserError {
    #[error("RDF parse error: {err}")]
    RDFParseError {
        #[from]
        err: RDFParseError,
    },

    #[error("Error converting Term to RDFNode: {term}")]
    TermToRDFNodeFailed { term: String },

    #[error("Expected RDFNode parsing node shape, found: {term}")]
    ExpectedRDFNodeNodeShape { term: String },

    #[error("Expected term as subject, found: {term}")]
    ExpectedSubject { term: String },

    #[error("Expected Value of `sh:or` to be a subject, found: {term}")]
    OrValueNoSubject { term: String },

    #[error("Expected Value of `sh:or` to be an object, found: {term}")]
    OrValueNoObject { term: String },

    #[error("Expected Value of `sh:and` to be a subject, found: {term}")]
    AndValueNoSubject { term: String },

    #[error("Expected Value of `sh:xone` to be a subject, found: {term}")]
    XOneValueNoSubject { term: String },

    #[error("Expected Value of `sh:not` to be an object, found: {term}")]
    NotValueNoObject { term: String },

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
