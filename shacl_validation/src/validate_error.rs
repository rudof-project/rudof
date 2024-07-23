use srdf::SRDFGraphError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Error obtaining the triples")]
    SRDFGraph(#[from] SRDFGraphError),
    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,
    #[error("TargetClass cannot be a Blank Node")]
    TargetClassBlankNode,
    #[error("TargetClass cannot be a Literal")]
    TargetClassLiteral,
}
