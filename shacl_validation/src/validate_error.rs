use thiserror::Error;

use crate::helpers::helper_error::SPARQLError;

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Expected a Subject but found {_0}")]
    ExpectedSubject(String),

    #[error("Expected a Literal but found {_0}")]
    ExpectedLiteral(String),

    #[error("Error during the SPARQL operation")]
    SRDF,

    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SPARQLError),

    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,

    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,

    #[error("Not yet implemented: {_0}")]
    NotImplemented(&'static str),
}
