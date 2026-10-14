use thiserror::Error;
use crate::ir::node_exprs::{IfExpression, Var};

#[derive(Error, Debug)]
pub enum NodeExprEvalError {
    #[error("The shnex:var value ({0}) is invalid, it must be a non-empty string")]
    EmptyVar(Var),

    #[error("The IfExpression ({0}) is invalid, shnex:else or shnex:then required")]
    IfBodyError(IfExpression),

    #[error("There is no focus node, but the expression ({0}) needs one")]
    MissingFNode(String),

    #[error("The focus node expression for ({0}) must evaluate to exactly one node, but got {1}")]
    MultipleFocusNodes(String, usize),

    #[error("Expected a numeric literal, but got: {0}")]
    NotNumeric(String),
}