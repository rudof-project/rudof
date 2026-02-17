use colored::*;
use itertools::Itertools;
use rdf::rdf_core::term::Object;
use serde::Serialize;
use tabled::settings::Modify;
use tabled::settings::Width;
use tabled::settings::object::Segment;

use crate::shapemap::ShapemapConfig;
use crate::shapemap::ShapemapError;
use crate::shapemap::ValidationStatus;
use crate::{Node, ir::shape_label::ShapeLabel};
use prefixmap::PrefixMap;
use serde::ser::{SerializeMap, SerializeSeq};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::Error;
use std::io::Write;
use tabled::{builder::Builder, settings::Style};

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

    pub fn ok_text(&self) -> String {
        self.config.ok_text()
    }

    pub fn fail_text(&self) -> String {
        self.config.fail_text()
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
    ) -> Result<(), Box<ShapemapError>> {
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
                            ) => *cell_status = ValidationStatus::Conformant(conformant_info.merge(conformant_info2)),
                            (
                                ValidationStatus::Conformant(_conformant_info),
                                ValidationStatus::NonConformant(_non_conformant_info),
                            ) => todo!(),
                            (ValidationStatus::Conformant(_conformant_info), ValidationStatus::Pending) => {},
                            (
                                ValidationStatus::Conformant(_conformant_info),
                                ValidationStatus::Inconsistent(_conformant_info2, _non_conformant_info2),
                            ) => todo!(),
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::Conformant(_conformant_info),
                            ) => todo!(),
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::NonConformant(_non_conformant_info2),
                            ) => {},
                            (ValidationStatus::NonConformant(_non_conformant_info), ValidationStatus::Pending) => {},
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::Inconsistent(_conformant_info2, _non_conformant_info2),
                            ) => todo!(),
                            (ValidationStatus::Pending, ValidationStatus::Conformant(conformant_info)) => {
                                *cell_status = ValidationStatus::Conformant(conformant_info)
                            },
                            (ValidationStatus::Pending, ValidationStatus::NonConformant(non_conformant_info)) => {
                                *cell_status = ValidationStatus::NonConformant(non_conformant_info)
                            },
                            (ValidationStatus::Pending, ValidationStatus::Pending) => {},
                            (
                                ValidationStatus::Pending,
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                            ) => todo!(),
                            (
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                                ValidationStatus::Conformant(_conformant_info2),
                            ) => todo!(),
                            (
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                                ValidationStatus::NonConformant(_non_conformant_info2),
                            ) => todo!(),
                            (
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                                ValidationStatus::Pending,
                            ) => todo!(),
                            (
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                                ValidationStatus::Inconsistent(_conformant_info2, _non_conformant_info2),
                            ) => todo!(),
                        };
                        ok()
                    },
                    Entry::Vacant(v) => {
                        v.insert(status);
                        ok()
                    },
                }
            },
            Entry::Vacant(v) => {
                let mut map = HashMap::new();
                map.insert(shape_label, status);
                v.insert(map);
                ok()
            },
        }?;
        ok()
    }

    pub fn get_info(&self, node: &Node, label: &ShapeLabel) -> Option<ValidationStatus> {
        match self.result.get(node) {
            Some(shapes) => shapes.get(label).cloned(),
            None => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Node, &ShapeLabel, &ValidationStatus)> {
        self.result
            .iter()
            .flat_map(|(node, shapes)| shapes.iter().map(move |(shape, status)| (node, shape, status)))
    }

    pub fn as_csv<W: Write>(&self, writer: W, sort_mode: SortMode, with_details: bool) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_writer(writer);
        wtr.write_record(["node", "shape", "status", "details"])?;

        let cmp = self.get_comparator(sort_mode);
        for (node, label, status) in self.iter().sorted_by(cmp) {
            let node_label = show_node(node, &self.nodes_prefixmap());
            let shape_label = show_shapelabel(label, &self.shapes_prefixmap());
            let details;
            let status_label;
            match status {
                ValidationStatus::Conformant(conformant_info) => {
                    details = conformant_info.to_string();
                    status_label = match self.ok_color() {
                        None => ColoredString::from(self.ok_text()),
                        Some(color) => self.ok_text().color(color).to_owned(),
                    };
                },
                ValidationStatus::NonConformant(non_conformant_info) => {
                    details = non_conformant_info.to_string();
                    status_label = match self.fail_color() {
                        None => ColoredString::from(self.fail_text()),
                        Some(color) => self.fail_text().color(color).to_owned(),
                    };
                },
                ValidationStatus::Pending => {
                    details = "".to_owned();
                    status_label = "Pending".color(self.pending_color().unwrap()).to_owned();
                },
                ValidationStatus::Inconsistent(ci, nci) => {
                    details = format!("Conformant: {ci}, Non-conformant: {nci}");
                    status_label = "Inconsistent".color(self.pending_color().unwrap()).to_owned();
                },
            };
            if with_details {
                wtr.write_record([node_label, shape_label, status_label.to_string(), details])?;
            } else {
                wtr.write_record([node_label, shape_label, status_label.to_string()])?;
            }
        }
        wtr.flush()?;
        Ok(())
    }

    pub fn as_table<W: Write>(
        &self,
        mut writer: W,
        sort_mode: SortMode,
        with_details: bool,
        terminal_width: usize,
    ) -> Result<(), Error> {
        let mut builder = Builder::default();
        if with_details {
            builder.push_record(["Node", "Shape", "Status", "Details"]);
        } else {
            builder.push_record(["Node", "Shape", "Status"]);
        }

        let cmp = self.get_comparator(sort_mode);
        for (node, label, status) in self.iter().sorted_by(cmp) {
            let node_label = show_node(node, &self.nodes_prefixmap());
            let shape_label = show_shapelabel(label, &self.shapes_prefixmap());
            let details;
            let status_label;
            match status {
                ValidationStatus::Conformant(conformant_info) => {
                    details = conformant_info.to_string();
                    status_label = match self.ok_color() {
                        None => ColoredString::from(self.ok_text()),
                        Some(color) => self.ok_text().color(color).to_owned(),
                    };
                },
                ValidationStatus::NonConformant(non_conformant_info) => {
                    details = non_conformant_info.to_string();
                    status_label = match self.fail_color() {
                        None => ColoredString::from(self.fail_text()),
                        Some(color) => self.fail_text().color(color).to_owned(),
                    };
                },
                ValidationStatus::Pending => {
                    details = "".to_owned();
                    status_label = "Pending".color(self.pending_color().unwrap()).to_owned();
                },
                ValidationStatus::Inconsistent(ci, nci) => {
                    details = format!("Conformant: {ci}, Non-conformant: {nci}");
                    status_label = "Inconsistent".color(self.pending_color().unwrap()).to_owned();
                },
            };
            if with_details {
                builder.push_record([node_label, shape_label, status_label.to_string(), details]);
            } else {
                builder.push_record([node_label, shape_label, status_label.to_string()]);
            }
        }
        let mut table = builder.build();
        table.with(Style::modern_rounded());
        table.with(Modify::new(Segment::all()).with(Width::wrap(terminal_width)));
        writeln!(writer, "{table}")?;
        Ok(())
    }

    fn get_comparator(
        &self,
        sort_mode: SortMode,
    ) -> impl FnMut(&(&Node, &ShapeLabel, &ValidationStatus), &(&Node, &ShapeLabel, &ValidationStatus)) -> std::cmp::Ordering
    {
        match sort_mode {
            SortMode::Node => |a: &(&Node, &ShapeLabel, &ValidationStatus),
                               b: &(&Node, &ShapeLabel, &ValidationStatus)| {
                a.0.cmp(b.0).then(a.1.cmp(b.1))
            },
            SortMode::Shape => |a: &(&Node, &ShapeLabel, &ValidationStatus),
                                b: &(&Node, &ShapeLabel, &ValidationStatus)| {
                a.1.cmp(b.1).then(a.0.cmp(b.0))
            },
            SortMode::Status => |a: &(&Node, &ShapeLabel, &ValidationStatus),
                                 b: &(&Node, &ShapeLabel, &ValidationStatus)| {
                a.2.code().cmp(&b.2.code()).then(a.0.cmp(b.0)).then(a.1.cmp(b.1))
            },
            SortMode::Details => |a: &(&Node, &ShapeLabel, &ValidationStatus),
                                  b: &(&Node, &ShapeLabel, &ValidationStatus)| {
                a.2.reason().cmp(&b.2.reason()).then(a.0.cmp(b.0)).then(a.1.cmp(b.1))
            },
        }
    }
}

fn ok() -> Result<(), Box<ShapemapError>> {
    Ok(())
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
            let result_aux = ResultSerializer { node, shape, status };
            seq.serialize_element(&result_aux)?;
        }
        seq.end()
    }
}

pub enum SortMode {
    Node,
    Shape,
    Status,
    Details,
}
