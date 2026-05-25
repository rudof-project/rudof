use crate::ast::error::ASTError;
use crate::error::IRError;
use rudof_rdf::rdf_core::{BuildRDF, RDFError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaclParserError {
    #[error(transparent)]
    ASTError(#[from] Box<ASTError>),

    #[error(transparent)]
    RDFError(#[from] Box<RDFError>),

    #[error("Expected Value of `{iri}` to be a {expected}, but found: {found}")]
    ValueNotExpected {
        iri: String,
        expected: String,
        found: String,
    },

    #[error("Expected term as subject, found {term} in {context}")]
    ExpectedSubject { term: String, context: String },

    #[error("An error occured while searching triples: {0}")]
    TriplesLookupError(String),

    #[error("Expected NodeKind, found: {0}")]
    ExpectedNodeKind(String),

    #[error("Unknown NodeKind, found: {0}")]
    UnknownNodeKind(String),
}

impl From<ASTError> for ShaclParserError {
    fn from(value: ASTError) -> Self {
        Self::ASTError(Box::new(value))
    }
}

impl From<RDFError> for ShaclParserError {
    fn from(value: RDFError) -> Self {
        Self::RDFError(Box::new(value))
    }
}

#[derive(Debug, Error)]
pub enum ShaclWriterError {
    #[error(transparent)]
    IRError(#[from] Box<IRError>),

    #[error("Unable to serialize RDF: {0}")]
    SerializationError(String),
}

impl ShaclWriterError {
    pub fn from_rdf_err<RDF: BuildRDF>(err: RDF::Err) -> Self {
        Self::SerializationError(err.to_string())
    }
}

impl From<IRError> for ShaclWriterError {
    fn from(value: IRError) -> Self {
        Self::IRError(Box::new(value))
    }
}
