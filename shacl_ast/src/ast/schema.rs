use std::{collections::HashMap, fmt::Display};

use crate::{node_shape::NodeShape, shape::Shape, ShaclError};
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use srdf::Object;

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

    pub fn add_node_shapes(&mut self, ns: Vec<NodeShape>) -> Result<(), ShaclError> {
        for node_shape in ns.iter() {
            let id = node_shape.id();
            match id {
                Object::Iri { iri } => {
                    self.shapes
                        .insert(IriRef::Iri(iri), Shape::NodeShape(node_shape.clone()));
                }
                _ => return Err(ShaclError::NodeShapeIdNotIri { id }),
            }
        }
        Ok(())
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
