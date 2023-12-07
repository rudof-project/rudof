use std::iter::once;

use shex_ast::{object_value::ObjectValue, ShapeExprLabel};
use srdf::SRDF;

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

    pub fn iter_node_shape<S>(&self, 
        rdf: &S) -> impl Iterator<Item=(&ObjectValue, &ShapeExprLabel)> 
    where S: SRDF {
       self.node_selector.iter_node(rdf)
       .flat_map(move |node| 
        self.shape_selector.iter_shape().flat_map(move |label| 
            once((node, label))
        ))
    }
}
