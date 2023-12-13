use std::fmt::Display;

use iri_s::IriS;

use crate::{ast::NodeConstraint as AstNodeConstraint, Cond};

use super::{node_kind::NodeKind, value_set_value::ValueSetValue, xs_facet::XsFacet};

/// Represents compiled node constraints
#[derive(Debug, PartialEq, Clone)]
pub struct NodeConstraint {
    source: AstNodeConstraint,
    cond: Cond,
}

impl NodeConstraint {
    pub fn new(nc: AstNodeConstraint, cond: Cond) -> Self {
        NodeConstraint { source: nc, cond }
    }

    pub fn cond(&self) -> Cond {
        self.cond.clone()
    }
}

impl Display for NodeConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeConstraint: {self:?}")
    }
}
