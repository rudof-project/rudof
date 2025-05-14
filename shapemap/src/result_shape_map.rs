use colored::*;
use serde::Serialize;
use srdf::Object;

use crate::ShapemapConfig;
use crate::ShapemapError;
use crate::ValidationStatus;
use prefixmap::PrefixMap;
use serde::ser::{SerializeMap, SerializeSeq};
use shex_ast::{ir::shape_label::ShapeLabel, Node};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

/// Contains a map of the results obtained after applying ShEx validation
#[derive(Debug, PartialEq, Default, Clone)]
pub struct ResultShapeMap {
    result: HashMap<Node, HashMap<ShapeLabel, ValidationStatus>>,

    config: ShapemapConfig,
}

impl ResultShapeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ok_color(&self) -> Option<Color> {
        self.config.ok_color()
    }

    pub fn fail_color(&self) -> Option<Color> {
        self.config.fail_color()
    }

    pub fn pending_color(&self) -> Option<Color> {
        self.config.pending_color()
    }

    pub fn set_ok_color(&mut self, color: Color) {
        self.config.set_ok_color(color);
    }

    pub fn set_fail_color(&mut self, color: Color) {
        self.config.set_fail_color(color);
    }

    pub fn set_pending_color(&mut self, color: Color) {
        self.config.set_pending_color(color)
    }

    pub fn nodes_prefixmap(&self) -> PrefixMap {
        self.config.nodes_prefixmap()
    }

    pub fn shapes_prefixmap(&self) -> PrefixMap {
        self.config.shapes_prefixmap()
    }

    pub fn with_nodes_prefixmap(mut self, prefixmap: &PrefixMap) -> Self {
        self.config = self.config.with_nodes_prefixmap(&prefixmap.clone());
        self
    }

    pub fn with_shapes_prefixmap(mut self, prefixmap: &PrefixMap) -> Self {
        self.config = self.config.with_shapes_prefixmap(&prefixmap.clone());
        self
    }

    pub fn add_result(
        &mut self,
        node: Node,
        shape_label: ShapeLabel,
        status: ValidationStatus,
    ) -> Result<(), ShapemapError> {
        let _cn = node.clone();
        let _sl = shape_label.clone();
        match self.result.entry(node) {
            Entry::Occupied(mut c) => {
                let map = c.get_mut();
                match map.entry(shape_label) {
                    Entry::Occupied(mut c) => {
                        let cell_status = c.get_mut();
                        match (cell_status.clone(), status) {
                            (
                                ValidationStatus::Conformant(conformant_info),
                                ValidationStatus::Conformant(conformant_info2),
                            ) => {
                                *cell_status = ValidationStatus::Conformant(
                                    conformant_info.merge(conformant_info2),
                                )
                            }
                            (
                                ValidationStatus::Conformant(_conformant_info),
                                ValidationStatus::NonConformant(_non_conformant_info),
                            ) => todo!(),
                            (
                                ValidationStatus::Conformant(_conformant_info),
                                ValidationStatus::Pending,
                            ) => {}
                            (
                                ValidationStatus::Conformant(_conformant_info),
                                ValidationStatus::Inconsistent(
                                    _conformant_info2,
                                    _non_conformant_info2,
                                ),
                            ) => todo!(),
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::Conformant(_conformant_info),
                            ) => todo!(),
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::NonConformant(_non_conformant_info2),
                            ) => {}
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::Pending,
                            ) => {}
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::Inconsistent(
                                    _conformant_info2,
                                    _non_conformant_info2,
                                ),
                            ) => todo!(),
                            (
                                ValidationStatus::Pending,
                                ValidationStatus::Conformant(conformant_info),
                            ) => *cell_status = ValidationStatus::Conformant(conformant_info),
                            (
                                ValidationStatus::Pending,
                                ValidationStatus::NonConformant(non_conformant_info),
                            ) => {
                                *cell_status = ValidationStatus::NonConformant(non_conformant_info)
                            }
                            (ValidationStatus::Pending, ValidationStatus::Pending) => {}
                            (
                                ValidationStatus::Pending,
                                ValidationStatus::Inconsistent(
                                    _conformant_info,
                                    _non_conformant_info,
                                ),
                            ) => todo!(),
                            (
                                ValidationStatus::Inconsistent(
                                    _conformant_info,
                                    _non_conformant_info,
                                ),
                                ValidationStatus::Conformant(_conformant_info2),
                            ) => todo!(),
                            (
                                ValidationStatus::Inconsistent(
                                    _conformant_info,
                                    _non_conformant_info,
                                ),
                                ValidationStatus::NonConformant(_non_conformant_info2),
                            ) => todo!(),
                            (
                                ValidationStatus::Inconsistent(
                                    _conformant_info,
                                    _non_conformant_info,
                                ),
                                ValidationStatus::Pending,
                            ) => todo!(),
                            (
                                ValidationStatus::Inconsistent(
                                    _conformant_info,
                                    _non_conformant_info,
                                ),
                                ValidationStatus::Inconsistent(
                                    _conformant_info2,
                                    _non_conformant_info2,
                                ),
                            ) => todo!(),
                        };
                        Ok(())
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
                show_node(node, &self.nodes_prefixmap()),
                show_shapelabel(label, &self.shapes_prefixmap())
            );
            match status {
                ValidationStatus::Conformant(conformant_info) => {
                    let node_label = match self.ok_color() {
                        None => ColoredString::from(node_label),
                        Some(color) => node_label.color(color),
                    };
                    write!(f, "{node_label} -> OK, reason: {conformant_info}")?;
                }
                ValidationStatus::NonConformant(non_conformant_info) => {
                    let node_label = match self.fail_color() {
                        None => ColoredString::from(node_label),
                        Some(color) => node_label.color(color),
                    };
                    write!(f, "{node_label} -> Fail, reason: {non_conformant_info}")?;
                }
                ValidationStatus::Pending => {
                    let node_label = match self.pending_color() {
                        None => ColoredString::from(node_label),
                        Some(color) => node_label.color(color),
                    };
                    write!(f, "{node_label} -> Pending")?
                }
                ValidationStatus::Inconsistent(conformant, inconformant) => {
                    let node_label = match self.pending_color() {
                        None => ColoredString::from(node_label),
                        Some(color) => node_label.color(color),
                    };
                    write!(f, "{node_label} -> Inconsistent, conformant: {conformant}, non-conformant: {inconformant}")?
                }
            }
        }
        Ok(())
    }
}

struct ResultSerializer<'a> {
    node: &'a Node,
    shape: &'a ShapeLabel,
    status: &'a ValidationStatus,
}

impl Serialize for ResultSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("node", &self.node.to_string())?;
        map.serialize_entry("shape", &self.shape.to_string())?;
        map.serialize_entry("status", &self.status.code())?;
        map.serialize_entry("appInfo", &self.status.app_info())?;
        map.serialize_entry("reason", &self.status.reason())?;
        map.end()
    }
}

impl Serialize for ResultShapeMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.result.len()))?;
        for (node, shape, status) in self.iter() {
            let result_aux = ResultSerializer {
                node,
                shape,
                status,
            };
            seq.serialize_element(&result_aux)?;
        }
        seq.end()
    }
}
