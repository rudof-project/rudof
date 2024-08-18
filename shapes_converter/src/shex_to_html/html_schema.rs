use std::collections::{hash_map::Entry, HashMap};

use prefixmap::PrefixMap;
use serde::Serialize;

use super::{HtmlShape, NodeId, ShEx2HtmlConfig};
use crate::ShEx2HtmlError;

#[derive(Debug, PartialEq, Default)]
pub struct HtmlSchema {
    labels_counter: usize,
    labels: HashMap<String, NodeId>,
    shapes: HashMap<NodeId, HtmlShape>,
    prefixmap: PrefixMap,
}

impl HtmlSchema {
    pub fn new() -> HtmlSchema {
        Default::default()
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    /// Tries to get a node from a label. If it exists returns the node and true, otherwise, adds the node and returns false
    pub fn get_node_adding_label(&mut self, label: &str) -> (NodeId, bool) {
        match self.labels.entry(label.to_string()) {
            Entry::Occupied(c) => (*c.get(), true),
            Entry::Vacant(v) => {
                self.labels_counter += 1;
                let n = NodeId::new(self.labels_counter);
                v.insert(n);
                (n, false)
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
        self.shapes.values()
    }

    pub fn to_landing_html_schema(&self, config: &ShEx2HtmlConfig) -> LandingHtmlSchema {
        let mut shapes = Vec::new();
        for shape in self.shapes.values() {
            shapes.push(ShapeRef::new(
                shape.name().name().as_str(),
                shape.name().as_relative_href().unwrap_or_default().as_str(),
                shape.name().label().unwrap_or_default().as_str(),
            ))
        }
        LandingHtmlSchema {
            title: config.title.clone(),
            shapes,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Default)]

pub struct LandingHtmlSchema {
    title: String,
    shapes: Vec<ShapeRef>,
}

#[derive(Serialize, Debug, PartialEq, Default)]

pub struct ShapeRef {
    name: String,
    href: String,
    label: String,
}

impl ShapeRef {
    pub fn new(name: &str, href: &str, label: &str) -> ShapeRef {
        ShapeRef {
            name: name.to_string(),
            href: href.to_string(),
            label: label.to_string(),
        }
    }
}
