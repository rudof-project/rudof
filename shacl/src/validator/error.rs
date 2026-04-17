use std::io;
use std::io::Error;
use thiserror::Error;
use rudof_rdf::rdf_core::RDFError;
use rudof_rdf::rdf_impl::InMemoryGraphError;
use sparql_service::RdfDataError;
use crate::error::{IRError, ShaclParserError};
use crate::ir::ShapeLabelIdx;

pub use crate::validator::constraints::error::*;
pub use crate::validator::engine::error::*;
pub use crate::validator::report::error::*;

// TODO - Maybe move to validation module
// TODO - Check if all the SPARQL error can be merged in one and if not improve enum variant names for
// TODO - better readability. Also check with other cases like constraints
#[derive(Debug, Error)]
pub enum ValidationError {

    #[error("Shape not found for shape idx {idx}: {error}")]
    ShapeNotFound {
        idx: ShapeLabelIdx,
        error: String,
    },

    #[error("Obtaining rdfs:subClassOf of {term}: {error}")]
    SubClassOf {
        term: String,
        error: String,
    },

    #[error("Obtaining reifiers of triple {triple}: {error}")]
    ReifiersOfTriple {
        triple: String,
        error: String,
    },

    #[error("Obtaining instances of {term}: {error}")]
    InstanceOf {
        term: String,
        error: String,
    },

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
        err: InMemoryGraphError,
    },

    // TODO - Move to store
    #[error("Error when parsing the SHACL graph: {err}")]
    ShaclParser {
        #[from]
        err: ShaclParserError,
    },

    #[error("Error during the constraint evaluation")]
    Constraint(#[from] ConstraintError),
    // #[error("Error parsing the IRI")]
    // IriParse(#[from} IriParseError)

    #[error("Error during some I/O operation")]
    IO(#[from] Error),

    #[error("Error loading the Shapes")]
    Shapes(#[from] RDFError),

    #[error("Error creating the SPARQL endpoint")]
    SparqlCreation,

    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SparqlError),

    #[error("Error during the SPARQL operation: {msg}, source: {source}")]
    SparqlError {
        msg: String,
        source: SparqlError,
    },

    #[error("Constraint error in component {component}: {source}")]
    ConstraintError {
        component: String,
        source: ConstraintError,
    },

    #[error("Implicit class not found")]
    ImplicitClassNotFound,

    #[error("The provided mode is not supported for the {structure} structure")]
    UnsupportedMode {
        structure: String,
    },

    #[error(transparent)]
    SrdfHelper(#[from] SrdfError),

    #[error("TargetClass error: {msg}")]
    TargetClassError {
        msg: String,
    },

    #[error("Error during the compilation of the AST Schema, {err}")]
    CompiledShacl {
        #[from]
        err: IRError
    },

    #[error("Not yet implemented: {msg}")]
    NotImplemented {
        msg: String,
    },

    #[error(transparent)]
    RdfDataError (#[from] RdfDataError),

    #[error("Error obtaining triples with subject {subject} during evaluation: {error}, checking CLOSED")]
    TriplesWithSubject {
        subject: String,
        error: String,
    },

    #[error("Error obtaining triples with subject {subject} and predicate {predicate} during validation: {error}, checking REIFIER SHAPE")]
    TriplesWithSubjectPredicate {
        subject: String,
        predicate: String,
        error: String
    },

    #[error("Error building class instance index: {err}")]
    ClassIndexBuild {
        err: String,
    },
}

#[derive(Error, Debug)]
    #[error("Readiing SHACL Config path {path_name:?} error: {error:?}")]
pub enum ShaclConfigError {
    ReadingConfig {
        path_name: String,
        error: io::Error,
    },

    #[error("Reading SHACL config TOML from {path_name:?}. Error: {error:?}")]
    Toml {
        path_name: String,
        error: toml::de::Error
    }
}