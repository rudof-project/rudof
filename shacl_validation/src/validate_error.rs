use thiserror::Error;

use crate::helper::helper_error::SPARQLError;

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Error during the SPARQL operation")]
    SPARQLError(#[from] SPARQLError),
    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,
    #[error("TargetClass cannot be a Blank Node")]
    TargetClassBlankNode,
    #[error("TargetClass cannot be a Literal")]
    TargetClassLiteral,
}
