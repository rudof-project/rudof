use crate::error::{IRError, ShaclParserError};
use rudof_rdf::rdf_core::{RDFError, Rdf};
use rudof_rdf::rdf_impl::{OxigraphInMemoryError, OxigraphEndpointError};
use sparql_service::RdfDataError;
use std::io;
use std::io::Error;
use thiserror::Error;

// TODO - Maybe move to validation module
// TODO - Check if all the SPARQL error can be merged in one and if not improve enum variant names for
// TODO - better readability. Also check with other cases like constraints
#[derive(Debug, Error)]
pub enum ValidationError {
    #[cfg(feature = "sparql")]
    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Unsupported SHACL validation mode: {0}")]
    UnsupportedMode(String),

    #[error(transparent)]
    IRError(#[from] Box<IRError>),

    #[error("Graph error: {0}")]
    GraphError(String),

    #[error(transparent)]
    RDFError(#[from] Box<RDFError>),

    #[cfg(feature = "sparql")]
    #[error(transparent)]
    OxigraphEndpointError(#[from] Box<OxigraphEndpointError>),

    #[cfg(feature = "sparql")]
    #[error(transparent)]
    RdfDataError(#[from] Box<RdfDataError>),

    #[error(transparent)]
    OxigraphInMemoryError(#[from] Box<OxigraphInMemoryError>),

    #[error(transparent)]
    ShaclParserError(#[from] Box<ShaclParserError>),

    #[error("Cannot convert {0} into {1}")]
    CastError(String, String),

    #[error("Parsing error: the required field '{0}' is missing")]
    MissingRequiredField(String),

    #[error("Parsing error: the field '{field}' has an invalid IRI value: {value}")]
    InvalidIriValue { field: String, value: String },

    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBNode,

    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,
}

impl ValidationError {
    #[cfg(feature = "sparql")]
    pub fn ask_query_error<RDF: Rdf>(error: RDF::Err) -> Self {
        Self::QueryError(format!("ASK query failed: {error}"))
    }

    #[cfg(feature = "sparql")]
    pub fn select_query_error<RDF: Rdf>(error: RDF::Err) -> Self {
        Self::QueryError(format!("SELECT query failed: {error}"))
    }

    pub fn new_graph_error<RDF: Rdf>(err: RDF::Err) -> Self {
        Self::GraphError(err.to_string())
    }

    pub fn new_graph_error_ctx<RDF: Rdf>(err: RDF::Err, ctx: &str) -> Self {
        Self::GraphError(format!("{ctx}: {err}"))
    }
}

impl From<IRError> for ValidationError {
    fn from(value: IRError) -> Self {
        Self::IRError(Box::new(value))
    }
}

impl From<RDFError> for ValidationError {
    fn from(value: RDFError) -> Self {
        Self::RDFError(Box::new(value))
    }
}

#[cfg(feature = "sparql")]
impl From<OxigraphEndpointError> for ValidationError {
    fn from(value: OxigraphEndpointError) -> Self {
        Self::OxigraphEndpointError(Box::new(value))
    }
}

#[cfg(feature = "sparql")]
impl From<RdfDataError> for ValidationError {
    fn from(value: RdfDataError) -> Self {
        Self::RdfDataError(Box::new(value))
    }
}

impl From<ShaclParserError> for ValidationError {
    fn from(value: ShaclParserError) -> Self {
        Self::ShaclParserError(Box::new(value))
    }
}

impl From<OxigraphInMemoryError> for ValidationError {
    fn from(value: OxigraphInMemoryError) -> Self {
        Self::OxigraphInMemoryError(Box::new(value))
    }
}

#[derive(Error, Debug)]
pub enum ShaclConfigError {
    #[error(transparent)]
    IOError(#[from] Box<io::Error>),

    #[error(transparent)]
    UnmarshallError(#[from] Box<toml::de::Error>),
}

impl From<io::Error> for ShaclConfigError {
    fn from(value: Error) -> Self {
        Self::IOError(Box::new(value))
    }
}

impl From<toml::de::Error> for ShaclConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::UnmarshallError(Box::new(value))
    }
}
