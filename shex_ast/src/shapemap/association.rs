use crate::ShapeExprLabel;
use crate::shapemap::{NodeSelector, ShapeSelector, ShapemapError};
use rudof_rdf::rdf_core::query::QueryRDF;
use serde::Serialize;
use std::fmt::Display;
use std::iter::once;
use tracing::trace;

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

    pub fn iter_node_shape<'a, S>(
        &'a self,
        rdf: &'a S,
    ) -> Result<impl Iterator<Item = (S::Term, &'a ShapeExprLabel)>, ShapemapError>
    where
        S: QueryRDF,
    {
        let nodes = self.node_selector.nodes(rdf)?;
        trace!("Association nodes: {:?}", nodes);
        let iter = nodes.into_iter().flat_map(move |node| {
            self.shape_selector
                .iter_shape()
                .flat_map(move |label| once((node.clone(), label)))
        });
        Ok(iter)
    }
}

impl Display for Association {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.node_selector, self.shape_selector)
    }
}
