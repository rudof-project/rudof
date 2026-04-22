use crate::error::{IRError, ShaclParserError};
use crate::ir::ShapeLabelIdx;
use rudof_rdf::rdf_core::RDFError;
use rudof_rdf::rdf_impl::InMemoryGraphError;
use sparql_service::RdfDataError;
use std::io;
use std::io::Error;
use thiserror::Error;

pub use crate::validator::constraints::error::*;
pub use crate::validator::engine::error::*;
pub use crate::validator::report::error::*;

// TODO - Maybe move to validation module
// TODO - Check if all the SPARQL error can be merged in one and if not improve enum variant names for
// TODO - better readability. Also check with other cases like constraints
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Shape not found for shape idx {idx}: {error}")]
    ShapeNotFound { idx: ShapeLabelIdx, error: String },

    #[error("Obtaining rdfs:subClassOf of {term}: {error}")]
    SubClassOf { term: String, error: String },

    #[error("Obtaining reifiers of triple {triple}: {error}")]
    ReifiersOfTriple { triple: String, error: String },

    #[error("Obtaining instances of {term}: {error}")]
    InstanceOf { term: String, error: String },

    #[error("Obtaining objects for focus node {focus_node} and SHACL path: {shacl_path}: {error}")]
    ObjectsShaclPath {
        focus_node: String,
        shacl_path: String,
        error: String,
    },

    #[error("Error during the SPARQL operation")]
    Srdf,

    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBNode,

    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,

    // TODO - Move to store
    #[error("Error when working with the SRDFGraph: {err}")]
    Graph {
        #[from]
        err: Box<InMemoryGraphError>,
    },

    // TODO - Move to store
    #[error("Error when parsing the SHACL graph: {err}")]
    ShaclParser {
        #[from]
        err: Box<ShaclParserError>,
    },

    #[error("Error during the constraint evaluation")]
    Constraint(#[from] ConstraintError),
    // #[error("Error parsing the IRI")]
    // IriParse(#[from} IriParseError)
    #[error("Error during some I/O operation")]
    IO(#[from] Error),

    #[error("Error loading the Shapes")]
    Shapes(#[from] Box<RDFError>),

    #[cfg(feature = "sparql")]
    #[error("Error creating the SPARQL endpoint")]
    SparqlCreation,

    #[cfg(feature = "sparql")]
    #[error("Error during the SPARQL operation")]
    Sparql(#[from] Box<SparqlError>),

    #[cfg(feature = "sparql")]
    #[error("Error during the SPARQL operation: {msg}, source: {source}")]
    SparqlError { msg: String, source: Box<SparqlError> },

    #[error("Constraint error in component {component}: {source}")]
    ConstraintError {
        component: String,
        source: Box<ConstraintError>,
    },

    #[error("Implicit class not found")]
    ImplicitClassNotFound,

    #[error("The provided mode is not supported for the {structure} structure")]
    UnsupportedMode { structure: String },

    #[error(transparent)]
    SrdfHelper(#[from] Box<SrdfError>),

    #[error("TargetClass error: {msg}")]
    TargetClassError { msg: String },

    #[error("Error during the compilation of the AST Schema, {err}")]
    CompiledShacl {
        #[from]
        err: Box<IRError>,
    },

    #[error("Not yet implemented: {msg}")]
    NotImplemented { msg: String },

    #[error(transparent)]
    RdfDataError(#[from] Box<RdfDataError>),

    #[error("Error obtaining triples with subject {subject} during evaluation: {error}, checking CLOSED")]
    TriplesWithSubject { subject: String, error: String },

    #[error(
        "Error obtaining triples with subject {subject} and predicate {predicate} during validation: {error}, checking REIFIER SHAPE"
    )]
    TriplesWithSubjectPredicate {
        subject: String,
        predicate: String,
        error: String,
    },

    #[error("Error building class instance index: {err}")]
    ClassIndexBuild { err: String },
}

impl From<IRError> for ValidationError {
    fn from(value: IRError) -> Self {
        Self::CompiledShacl { err: Box::new(value) }
    }
}

impl From<InMemoryGraphError> for ValidationError {
    fn from(value: InMemoryGraphError) -> Self {
        Self::Graph { err: Box::new(value) }
    }
}

impl From<ShaclParserError> for ValidationError {
    fn from(value: ShaclParserError) -> Self {
        Self::ShaclParser { err: Box::new(value) }
    }
}

impl From<RdfDataError> for ValidationError {
    fn from(value: RdfDataError) -> Self {
        Self::RdfDataError(Box::new(value))
    }
}

#[derive(Error, Debug)]
pub enum ShaclConfigError {
    #[error("Reading SHACL Config path {path_name:?} error: {error:?}")]
    ReadingConfig { path_name: String, error: io::Error },

    #[error("Reading SHACL config TOML from {path_name:?}. Error: {error:?}")]
    Toml { path_name: String, error: toml::de::Error },
}
