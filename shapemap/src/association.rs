use crate::{NodeSelector, ShapeSelector};

/// Combines a [`NodeSelector`] with a [`ShapeExprLabel`]
#[derive(Debug, PartialEq)]
pub struct Association {
    pub node_selector: NodeSelector,
    pub shape_selector: ShapeSelector,
}

impl Association {
    pub fn new(node_selector: NodeSelector, shape_selector: ShapeSelector) -> Self {
        Association {
            node_selector,
            shape_selector,
        }
    }
}
