use std::{collections::HashMap, fmt::Display};

use crate::{node_shape::NodeShape, shape::Shape, ShaclError};
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use srdf::{Object, RDFNode};

#[derive(Debug, Clone, Default)]
pub struct Schema {
    imports: Vec<IriS>,
    entailments: Vec<IriS>,
    shapes: HashMap<RDFNode, Shape>,
    prefixmap: PrefixMap,
    base: Option<IriS>
}

impl Schema {
    pub fn new() -> Schema {
        Schema::default()
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_shapes(mut self, shapes: HashMap<RDFNode, Shape>) -> Self {
        self.shapes = shapes;
        self
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> Option<IriS> {
        self.base.clone()
    }

}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, shape) in self.shapes.iter() {
            writeln!(f, "{id} -> {shape}")?;
        }
        Ok(())
    }
}
