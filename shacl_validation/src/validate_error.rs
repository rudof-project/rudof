use srdf::RDFParseError;
use thiserror::Error;

use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::helper_error::SPARQLError;

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Expected a Subject but found a term: {_0}")]
    ExpectedSubject(String),

    #[error("Error during the SPARQL operation")]
    SRDF,

    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,
    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,
    #[error("Error during the constraint evaluation")]
    Constraint(#[from] ConstraintError),
    #[error("Error during some I/O operation")]
    IO(#[from] std::io::Error),
    #[error("Error loading the Shapes")]
    Shapes(#[from] RDFParseError),
    #[error("Error creating the SPARQL endpoint")]
    SPARQLCreation,
    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SPARQLError),
    #[error("Implicit class not found")]
    ImplicitClassNotFound,
    #[error("The provided mode is not supported for the {} structure", ._0)]
    UnsupportedMode(String),
    #[error("Not yet implemented: {msg}")]
    NotImplemented { msg: String },
}
