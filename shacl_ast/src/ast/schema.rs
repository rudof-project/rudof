use std::collections::HashMap;

use iri_s::IriS;
use prefixmap::{PrefixMap, IriRef};
use crate::{shape::Shape, node_shape::NodeShape};

#[derive(Debug, Clone, Default)]
pub struct Schema {
    imports: Vec<IriS>,
    entailments: Vec<IriS>,
    shapes: HashMap<IriRef, Shape>,
    prefixmap: PrefixMap,
}

impl Schema {
    pub fn new() -> Schema {
        Schema::default()
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn add_node_shapes(&mut self, ns: Vec<NodeShape>) -> Self {
        todo!()
    }
}