use srdf::RdfParseError;
use thiserror::Error;

use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::helper_error::SPARQLError;
use crate::helpers::helper_error::SRDFError;

#[derive(Error, Debug)]
pub enum ValidateError {
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
    Shapes(#[from] RdfParseError),
    #[error("Error creating the SPARQL endpoint")]
    SPARQLCreation,
    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SPARQLError),
    #[error("Implicit class not found")]
    ImplicitClassNotFound,
    #[error("The provided mode is not supported for the {} structure", ._0)]
    UnsupportedMode(String),
    #[error(transparent)]
    SrdfHelper(#[from] SRDFError),
    #[error("Not yet implemented: {msg}")]
    NotImplemented { msg: String },
}
