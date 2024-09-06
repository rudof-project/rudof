use std::{
    collections::{hash_map::Entry, HashMap},
    time::SystemTime,
};

use chrono::{DateTime, Utc};
use prefixmap::PrefixMap;

use super::{HtmlShape, NodeId, ShEx2HtmlConfig};
use crate::{
    landing_html_template::{LandingHtmlTemplate, ShapeRef},
    ShEx2HtmlError,
};

#[derive(Debug, PartialEq, Default)]
pub struct HtmlSchema {
    labels_counter: usize,
    labels: HashMap<String, NodeId>,
    shapes: HashMap<NodeId, HtmlShape>,
    prefixmap: PrefixMap,
    svg_schema: String,
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

    pub fn set_svg_schema(&mut self, svg_schema: &str) {
        self.svg_schema = svg_schema.to_string()
    }

    pub fn add_component(
        &mut self,
        node: NodeId,
        component: HtmlShape,
    ) -> Result<(), ShEx2HtmlError> {
        match self.shapes.entry(node) {
            Entry::Occupied(mut v) => {
                v.get_mut().merge(&component);
                Ok(())
            }
            Entry::Vacant(v) => {
                v.insert(component);
                Ok(())
            }
        }
    }

    pub fn shapes(&self) -> impl Iterator<Item = &HtmlShape> {
        self.shapes.values()
    }

    pub fn to_landing_html_schema(&self, config: &ShEx2HtmlConfig) -> LandingHtmlTemplate {
        let mut shapes = Vec::new();
        for shape in self.shapes.values() {
            shapes.push(ShapeRef::new(
                shape.name().name().as_str(),
                shape.name().as_relative_href().unwrap_or_default().as_str(),
                shape.name().label().unwrap_or_default().as_str(),
            ))
        }
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let curr_time = SystemTime::now();
        let dt: DateTime<Utc> = curr_time.into();
        let created_time = dt.format("%Y-&m-%d %H:%M:%S").to_string();

        LandingHtmlTemplate {
            title: config.title.clone(),
            rudof_version: VERSION.to_string(),
            created_time,
            svg_schema: self.svg_schema.clone(),
            shapes,
        }
    }
}
