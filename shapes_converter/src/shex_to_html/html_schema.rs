use std::{
    collections::{hash_map::Entry, HashMap},
    path::Path,
};

use prefixmap::PrefixMap;

use crate::{ShEx2HtmlConfig, ShEx2HtmlError};

use super::{HtmlShape, Name, NodeId};

#[derive(Debug, PartialEq)]
pub struct HtmlSchema {
    labels_counter: usize,
    labels: HashMap<Name, NodeId>,
    shapes: HashMap<NodeId, HtmlShape>,
    prefixmap: PrefixMap,
}

impl HtmlSchema {
    pub fn new() -> HtmlSchema {
        HtmlSchema {
            labels_counter: 0,
            labels: HashMap::new(),
            shapes: HashMap::new(),
            prefixmap: PrefixMap::new(),
        }
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn add_label(&mut self, label: &Name) -> NodeId {
        match self.labels.entry(label.clone()) {
            Entry::Occupied(c) => *c.get(),
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let n = NodeId::new(self.labels_counter);
                v.insert(n);
                n
            }
        }
    }

    pub fn add_component(
        &mut self,
        node: NodeId,
        component: HtmlShape,
    ) -> Result<(), ShEx2HtmlError> {
        match self.shapes.entry(node) {
            Entry::Occupied(c) => Err(ShEx2HtmlError::NodeIdHasShape {
                node_id: node,
                shape: Box::new(c.get().clone()),
            }),
            Entry::Vacant(v) => {
                v.insert(component);
                Ok(())
            }
        }
    }

    pub fn shapes(&self) -> impl Iterator<Item = &HtmlShape> {
        self.shapes.iter().map(|(_, s)| s)
    }
}
