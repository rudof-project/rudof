use crate::shapemap::{Association, NodeSelector, ShapeSelector, ShapemapError};
use crate::{Node, ShapeExprLabel, ir::shape_label::ShapeLabel, object_value::ObjectValue};
use prefixmap::PrefixMap;
use rudof_rdf::rdf_core::query::QueryRDF;
use serde::Serialize;
use std::fmt::Display;
use tracing::trace;

#[derive(Debug, Default, PartialEq, Clone, Serialize)]
pub struct QueryShapeMap {
    associations: Vec<Association>,
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,
}

impl QueryShapeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nodes_prefixmap(&self) -> PrefixMap {
        self.nodes_prefixmap.clone()
    }

    pub fn shapes_prefixmap(&self) -> PrefixMap {
        self.shapes_prefixmap.clone()
    }

    pub fn with_nodes_prefixmap(mut self, prefixmap: &PrefixMap) -> Self {
        self.nodes_prefixmap = prefixmap.clone();
        self
    }

    pub fn with_shapes_prefixmap(mut self, prefixmap: &PrefixMap) -> Self {
        self.shapes_prefixmap = prefixmap.clone();
        self
    }

    pub fn add_association(&mut self, node_selector: NodeSelector, shape_selector: ShapeSelector) {
        let association = Association::new(node_selector, shape_selector);
        self.associations.push(association)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Association> + '_ {
        self.associations.iter()
    }

    pub fn node_shapes<'a, R>(&'a self, rdf: &'a R) -> Result<Vec<(R::Term, &'a ShapeExprLabel)>, ShapemapError>
    where
        R: QueryRDF,
    {
        trace!("node_shapes, pairs from QueryShapeMap: {}", self);
        let mut result = Vec::new();
        for assoc in self.iter() {
            trace!("Processing association: {}", assoc);
            let node_shapes = assoc.iter_node_shape(rdf)?;
            for pair in node_shapes {
                result.push(pair)
            }
        }
        Ok(result)
    }

    pub fn from_node_shape(node: &Node, shape: &ShapeLabel) -> Result<Self, crate::SchemaJsonError> {
        let mut sm = QueryShapeMap::new();
        let object_value: ObjectValue = node.try_into()?;
        let shape: ShapeExprLabel = shape.into();
        sm.add_association(NodeSelector::Node(object_value), ShapeSelector::label(shape));
        Ok(sm)
    }
}

impl Display for QueryShapeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.associations.iter();
        if let Some(first) = iter.next() {
            write!(f, "{first}")?;
            for assoc in iter {
                write!(f, ",\n{assoc}")?;
            }
        }
        Ok(())
    }
}
