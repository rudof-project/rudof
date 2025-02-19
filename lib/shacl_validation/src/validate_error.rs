use thiserror::Error;

use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::helper_error::SPARQLError;

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,

    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,

    #[error("Implicit class not found")]
    ImplicitClassNotFound,

    #[error("Error during the constraint evaluation")]
    Constraint(#[from] ConstraintError),

    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SPARQLError),

    #[error("Not yet implemented: {msg}")]
    NotImplemented { msg: String },

    #[error("Error in RDF operation: {0}")]
    Rdf(#[from] Box<dyn std::error::Error + Send + Sync>),
}
