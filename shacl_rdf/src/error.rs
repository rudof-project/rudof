use iri_s::error::IriSError;
use prefixmap::PrefixMapError;
use rdf::rdf_core::RDFError;
use shacl_ast::ShaclError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaclParserError {
    #[error("RDF parse error: {err}")]
    RDFParseError {
        #[from]
        err: RDFError,
    },

    #[error("Expected Value of `sh:reifierShape` to be a subject, found: {term}")]
    ReifierShapeNoSubject { term: String },

    #[error("Error converting Term to RDFNode: {term}")]
    TermToRDFNodeFailed { term: String },

    #[error("Expected RDFNode parsing node shape, found: {term}")]
    ExpectedRDFNodeNodeShape { term: String },

    #[error("Expected term as subject, found: {term} in {context}")]
    ExpectedSubject { term: String, context: String },

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

#[derive(Debug, Error)]
pub enum ShaclWriterError {
    #[error("IRI parsing error: {err}")]
    IriS {
        #[from]
        err: IriSError,
    },

    #[error("Prefix map error: {err}")]
    PrefixMap {
        #[from]
        err: PrefixMapError,
    },

    #[error("An error occured while writing RDF: {msg}")]
    Write { msg: String },

    #[error("An error occured while adding a prefix map to RDF: {msg}")]
    AddPrefixMap { msg: String },

    #[error("An error occured while adding a base to RDF: {msg}")]
    AddBase { msg: String },
}
