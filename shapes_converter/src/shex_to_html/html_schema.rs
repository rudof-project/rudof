use std::collections::{hash_map::Entry, HashMap};

use prefixmap::PrefixMap;
use serde::Serialize;

use super::{HtmlShape, Name, NodeId, ShEx2HtmlConfig};
use crate::ShEx2HtmlError;

#[derive(Debug, PartialEq, Default)]
pub struct HtmlSchema {
    labels_counter: usize,
    labels: HashMap<Name, NodeId>,
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
        self.shapes.values()
    }

    pub fn to_landing_html_schema(&self, config: &ShEx2HtmlConfig) -> LandingHtmlSchema {
        let mut shapes = Vec::new();
        for shape in self.shapes.values() {
            shapes.push(ShapeRef::new(
                shape.name().name().as_str(),
                shape.name().as_local_href().unwrap_or_default().as_str(),
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
}

impl ShapeRef {
    pub fn new(name: &str, href: &str) -> ShapeRef {
        ShapeRef {
            name: name.to_string(),
            href: href.to_string(),
        }
    }
}
