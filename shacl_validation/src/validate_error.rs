use thiserror::Error;

use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::helper_error::SPARQLError;

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Expected a Subject but found a Term: {_0}")]
    ExpectedSubject(String),

    #[error(transparent)]
    Constraint(#[from] ConstraintError),

    #[error("Error during the SPARQL operation")]
    SRDF,

    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SPARQLError),

    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,

    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,

    #[error("Not yet implemented: {msg}")]
    NotImplemented { msg: String },
}
