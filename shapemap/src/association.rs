use crate::{NodeSelector, ShapeSelector};
use serde::Serialize;
use shex_ast::{object_value::ObjectValue, ShapeExprLabel};
use srdf::NeighsRDF;
use std::iter::once;

/// Combines a [`NodeSelector`] with a [`ShapeExprLabel`]
#[derive(Debug, PartialEq, Clone, Serialize)]
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

    pub fn iter_node_shape<S>(
        &self,
        rdf: &S,
    ) -> impl Iterator<Item = (&ObjectValue, &ShapeExprLabel)>
    where
        S: NeighsRDF,
    {
        self.node_selector.iter_node(rdf).flat_map(move |node| {
            self.shape_selector
                .iter_shape()
                .flat_map(move |label| once((node, label)))
        })
    }
}
