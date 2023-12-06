use crate::{Association, NodeSelector, ShapeSelector};
use prefixmap::PrefixMap;

#[derive(Debug, PartialEq)]
pub struct QueryShapeMap {
    associations: Vec<Association>,
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,
}

impl QueryShapeMap {
    pub fn new() -> QueryShapeMap {
        QueryShapeMap {
            associations: Vec::new(),
            nodes_prefixmap: PrefixMap::new(),
            shapes_prefixmap: PrefixMap::new(),
        }
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
}
