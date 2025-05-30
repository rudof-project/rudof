use crate::{ast::NodeConstraint as AstNodeConstraint, Cond};
use std::fmt::Display;

/// Represents compiled node constraints
#[derive(Debug, PartialEq, Clone)]
pub struct NodeConstraint {
    source: AstNodeConstraint,
    cond: Cond,
    display: String,
}

impl NodeConstraint {
    pub fn new(nc: AstNodeConstraint, cond: Cond, display: String) -> Self {
        NodeConstraint {
            source: nc,
            cond,
            display,
        }
    }

    pub fn cond(&self) -> Cond {
        self.cond.clone()
    }
}

impl Display for NodeConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}
