use crate::{Association, NodeSelector, ShapeSelector};
use prefixmap::PrefixMap;
use shex_ast::{object_value::ObjectValue, ShapeExprLabel};
use srdf::SRDF;

#[derive(Debug, Default, PartialEq, Clone)]
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
    ) -> impl Iterator<Item = (&ObjectValue, &ShapeExprLabel)> + 'a
    where
        S: SRDF,
    {
        self.iter().flat_map(|assoc| assoc.iter_node_shape(rdf))
    }
}
