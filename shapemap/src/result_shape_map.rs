use colored::*;
use srdf::Object;

use crate::ShapemapError;
use crate::ValidationStatus;
use prefixmap::PrefixMap;
use shex_ast::{compiled::shape_label::ShapeLabel, Node};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

/// Contains a map of the results obtained after applying ShEx validation
#[derive(Debug, PartialEq, Clone)]
pub struct ResultShapeMap {
    result: HashMap<Node, HashMap<ShapeLabel, ValidationStatus>>,
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,
    ok_color: Option<Color>,
    fail_color: Option<Color>,
    pending_color: Option<Color>,
}

impl Default for ResultShapeMap {
    fn default() -> Self {
        Self {
            result: Default::default(),
            nodes_prefixmap: Default::default(),
            shapes_prefixmap: Default::default(),
            ok_color: Some(Color::Green),
            fail_color: Some(Color::Red),
            pending_color: Some(Color::Magenta),
        }
    }
}

impl ResultShapeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_ok_color(mut self, color: Color) {
        self.ok_color = Some(color);
    }

    pub fn set_fail_color(mut self, color: Color) {
        self.fail_color = Some(color);
    }

    pub fn set_pending_color(mut self, color: Color) {
        self.pending_color = Some(color)
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

    pub fn add_result(
        &mut self,
        node: Node,
        shape_label: ShapeLabel,
        status: ValidationStatus,
    ) -> Result<(), ShapemapError> {
        let cn = node.clone();
        let sl = shape_label.clone();
        match self.result.entry(node) {
            Entry::Occupied(mut c) => {
                let map = c.get_mut();
                match map.entry(shape_label) {
                    Entry::Occupied(c) => {
                        let old_status = c.get();
                        if *old_status != status {
                            Err(ShapemapError::InconsistentStatus {
                                node: cn,
                                label: sl,
                                old_status: old_status.clone(),
                                new_status: status,
                            })
                        } else {
                            Ok(())
                        }
                    }
                    Entry::Vacant(v) => {
                        v.insert(status);
                        Ok(())
                    }
                }
            }
            Entry::Vacant(v) => {
                let mut map = HashMap::new();
                map.insert(shape_label, status);
                v.insert(map);
                Ok(())
            }
        }?;
        Ok(())
    }

    pub fn get_info(&self, node: &Node, label: &ShapeLabel) -> Option<ValidationStatus> {
        match self.result.get(node) {
            Some(shapes) => shapes.get(label).cloned(),
            None => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Node, &ShapeLabel, &ValidationStatus)> {
        self.result.iter().flat_map(|(node, shapes)| {
            shapes
                .iter()
                .map(move |(shape, status)| (node, shape, status))
        })
    }
}

fn show_node(node: &Node, prefixmap: &PrefixMap) -> String {
    match node.as_object() {
        Object::Iri(iri) => prefixmap.qualify(iri),
        _ => format!("{node}"),
    }
}

fn show_shapelabel(shapelabel: &ShapeLabel, prefixmap: &PrefixMap) -> String {
    match shapelabel {
        ShapeLabel::Iri(iri) => prefixmap.qualify(iri),
        ShapeLabel::BNode(str) => format!("_:{str}"),
        ShapeLabel::Start => "Start".to_owned(),
    }
}

impl Display for ResultShapeMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for (node, label, status) in self.iter() {
            let node_label = format!(
                "{}@{}",
                show_node(node, &self.nodes_prefixmap),
                show_shapelabel(label, &self.shapes_prefixmap)
            );
            match status {
                ValidationStatus::Conformant(conformant_info) => {
                    let node_label = match self.ok_color {
                        None => ColoredString::from(node_label),
                        Some(color) => node_label.color(color),
                    };
                    write!(f, "{node_label} -> OK, reason: {conformant_info}")?;
                }
                ValidationStatus::NonConformant(non_conformant_info) => {
                    let node_label = match self.fail_color {
                        None => ColoredString::from(node_label),
                        Some(color) => node_label.color(color),
                    };
                    write!(f, "{node_label} -> Fail, reason: {non_conformant_info}")?;
                }
                ValidationStatus::Pending => {
                    let node_label = match self.pending_color {
                        None => ColoredString::from(node_label),
                        Some(color) => node_label.color(color),
                    };
                    write!(f, "{node_label} -> Pending")?
                }
            }
        }
        Ok(())
    }
}
