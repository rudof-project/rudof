use crate::error::{IRError, ShaclParserError};
use rudof_rdf::rdf_core::{RDFError, Rdf};
#[cfg(all(feature = "sparql", not(target_family = "wasm")))]
use rudof_rdf::rdf_impl::OxigraphEndpointError;
use rudof_rdf::rdf_impl::OxigraphInMemoryError;
#[cfg(feature = "sparql")]
use sparql_service::RdfDataError;
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

    #[cfg(all(feature = "sparql", not(target_family = "wasm")))]
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

    #[error("A sh:targetNode node expression has no focus node to read, but {0} reads one")]
    TargetNodeExprReadsFocus(String),

    #[error("sh:targetWhere must be a shape, and {0} is not one")]
    TargetWhereNotShape(String),

    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,

    #[error("Validating the shape {0} holds more than {1} results and evidences, the configured max_results")]
    TooManyResults(String, usize),
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

#[cfg(all(feature = "sparql", not(target_family = "wasm")))]
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
