use crate::shapemap::{Association, NodeSelector, ShapeSelector};
use crate::{Node, ShapeExprLabel, ir::shape_label::ShapeLabel, object_value::ObjectValue};
use prefixmap::PrefixMap;
use serde::Serialize;
use srdf::NeighsRDF;
use std::fmt::Display;

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

    pub fn iter_node_shape<'a, S>(
        &'a self,
        rdf: &'a S,
    ) -> impl Iterator<Item = (&'a ObjectValue, &'a ShapeExprLabel)> + 'a
    where
        S: NeighsRDF,
    {
        self.iter().flat_map(|assoc| assoc.iter_node_shape(rdf))
    }

    pub fn from_node_shape(
        node: &Node,
        shape: &ShapeLabel,
    ) -> Result<Self, crate::SchemaJsonError> {
        let mut sm = QueryShapeMap::new();
        let object_value: ObjectValue = node.try_into()?;
        let shape: ShapeExprLabel = shape.into();
        sm.add_association(
            NodeSelector::Node(object_value),
            ShapeSelector::label(shape),
        );
        Ok(sm)
    }
}

impl Display for QueryShapeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error)?
        )
    }
}
