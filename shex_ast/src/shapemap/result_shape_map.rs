use colored::*;
use itertools::Itertools;
use rudof_rdf::rdf_core::term::Object;
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

    pub fn ok_color(&self) -> &Color {
        self.config.ok_color()
    }

    pub fn ok_text(&self) -> &String {
        self.config.ok_text()
    }

    pub fn fail_text(&self) -> &String {
        self.config.fail_text()
    }

    pub fn fail_color(&self) -> &Color {
        self.config.fail_color()
    }

    pub fn pending_color(&self) -> &Color {
        self.config.pending_color()
    }

    pub fn set_ok_color(&mut self, color: Color) {
        self.config = std::mem::take(&mut self.config).with_ok_color(color);
    }

    pub fn set_fail_color(&mut self, color: Color) {
        self.config = std::mem::take(&mut self.config).with_fail_color(color);
    }

    pub fn set_pending_color(&mut self, color: Color) {
        self.config = std::mem::take(&mut self.config).with_pending_color(color);
    }

    pub fn nodes_prefixmap(&self) -> &PrefixMap {
        self.config.nodes_prefixmap()
    }

    pub fn shapes_prefixmap(&self) -> &PrefixMap {
        self.config.shapes_prefixmap()
    }

    pub fn with_nodes_prefixmap(mut self, prefixmap: &PrefixMap) -> Self {
        self.config = self.config.with_nodes_prefixmap(prefixmap.clone());
        self
    }

    pub fn with_shapes_prefixmap(mut self, prefixmap: &PrefixMap) -> Self {
        self.config = self.config.with_shapes_prefixmap(prefixmap.clone());
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
                            ) => todo!(
                                "Handle the case where we have a conformant status and a non-conformant status for the same node and shape"
                            ),
                            (ValidationStatus::Conformant(_conformant_info), ValidationStatus::Pending) => {},
                            (
                                ValidationStatus::Conformant(_conformant_info),
                                ValidationStatus::Inconsistent(_conformant_info2, _non_conformant_info2),
                            ) => todo!(
                                "Handle the case where we have a conformant status and an inconsistent status for the same node and shape"
                            ),
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::Conformant(_conformant_info),
                            ) => todo!(
                                "Handle the case where we have a non-conformant status and a conformant status for the same node and shape"
                            ),
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::NonConformant(_non_conformant_info2),
                            ) => {},
                            (ValidationStatus::NonConformant(_non_conformant_info), ValidationStatus::Pending) => {},
                            (
                                ValidationStatus::NonConformant(_non_conformant_info),
                                ValidationStatus::Inconsistent(_conformant_info2, _non_conformant_info2),
                            ) => todo!(
                                "Handle the case where we have a non-conformant status and an inconsistent status for the same node and shape"
                            ),
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
                            ) => todo!(
                                "Handle the case where we have a pending status and an inconsistent status for the same node and shape"
                            ),
                            (
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                                ValidationStatus::Conformant(_conformant_info2),
                            ) => todo!(
                                "Handle the case where we have an inconsistent status and a conformant status for the same node and shape"
                            ),
                            (
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                                ValidationStatus::NonConformant(_non_conformant_info2),
                            ) => todo!(
                                "Handle the case where we have an inconsistent status and a non-conformant status for the same node and shape"
                            ),
                            (
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                                ValidationStatus::Pending,
                            ) => todo!(
                                "Handle the case where we have an inconsistent status and a pending status for the same node and shape"
                            ),
                            (
                                ValidationStatus::Inconsistent(_conformant_info, _non_conformant_info),
                                ValidationStatus::Inconsistent(_conformant_info2, _non_conformant_info2),
                            ) => todo!(
                                "Handle the case where we have two inconsistent statuses for the same node and shape"
                            ),
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

    pub fn iter_sorted(&self, sort_mode: &SortMode) -> impl Iterator<Item = (&Node, &ShapeLabel, &ValidationStatus)> {
        self.iter().sorted_by(self.get_comparator(sort_mode))
    }

    pub fn as_csv<W: Write>(&self, writer: W, sort_mode: SortMode, with_details: bool) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_writer(writer);
        wtr.write_record(["node", "shape", "status", "details"])?;

        for (node, label, status) in self.iter_sorted(&sort_mode) {
            let node_label = show_node(node, self.nodes_prefixmap());
            let shape_label = show_shapelabel(label, self.shapes_prefixmap());
            // CSV is read by other programs, so the status is written without color: `colored`
            // decides by the process' stdout, which would put escape codes in a piped report
            let (details, status_label) = match status {
                ValidationStatus::Conformant(conformant_info) => (conformant_info.to_string(), self.ok_text().clone()),
                ValidationStatus::NonConformant(non_conformant_info) => {
                    (non_conformant_info.to_string(), self.fail_text().clone())
                },
                ValidationStatus::Pending => ("".to_owned(), "Pending".to_owned()),
                ValidationStatus::Inconsistent(ci, nci) => (
                    format!("Conformant: {ci}, Non-conformant: {nci}"),
                    "Inconsistent".to_owned(),
                ),
            };
            if with_details {
                wtr.write_record([node_label, shape_label, status_label, details])?;
            } else {
                wtr.write_record([node_label, shape_label, status_label])?;
            }
        }
        wtr.flush()?;
        Ok(())
    }

    pub fn as_table<W: Write>(
        &self,
        mut writer: W,
        sort_mode: Option<&SortMode>,
        with_details: Option<bool>,
        details_layout: Option<DetailsLayout>,
        terminal_width: Option<usize>,
    ) -> Result<(), Error> {
        let with_details = with_details.unwrap_or(false);
        let details_layout = details_layout.unwrap_or_default();
        let terminal_width = terminal_width.unwrap_or(80);
        let sort_mode = sort_mode.unwrap_or(&SortMode::Node);

        let mut rows = Vec::new();
        for (node, label, status) in self.iter_sorted(sort_mode) {
            let node_label = show_node(node, self.nodes_prefixmap());
            let shape_label = show_shapelabel(label, self.shapes_prefixmap());
            let (details, status_text, status_label) = match status {
                ValidationStatus::Conformant(conformant_info) => (
                    conformant_info.to_string(),
                    self.ok_text().clone(),
                    self.ok_text().color(*self.ok_color()),
                ),
                ValidationStatus::NonConformant(non_conformant_info) => (
                    non_conformant_info.to_string(),
                    self.fail_text().clone(),
                    self.fail_text().color(*self.fail_color()),
                ),
                ValidationStatus::Pending => (
                    "".to_owned(),
                    "Pending".to_owned(),
                    "Pending".color(*self.pending_color()),
                ),
                ValidationStatus::Inconsistent(ci, nci) => (
                    format!("Conformant: {ci}, Non-conformant: {nci}"),
                    "Inconsistent".to_owned(),
                    "Inconsistent".color(*self.pending_color()),
                ),
            };
            rows.push((node_label, shape_label, status_text, status_label.to_string(), details));
        }

        let mut builder = Builder::default();
        // The details of a result that don't fit in its row, written below the table
        let mut sections = Vec::new();
        if with_details && details_layout == DetailsLayout::Table {
            builder.push_record(["Node", "Shape", "Status", "Details"]);
            for (node_label, shape_label, _, status_label, details) in rows {
                builder.push_record([node_label, shape_label, status_label, details]);
            }
        } else if with_details {
            builder.push_record(["Node", "Shape", "Status", "Summary"]);
            // The summary column takes what the other columns and the borders leave: each of the
            // four columns takes its width plus a space on each side, and there are five borders.
            let column_width = |header: &str, cell: fn(&(String, String, String, String, String)) -> &String| {
                rows.iter()
                    .map(|row| cell(row).chars().count())
                    .chain([header.len()])
                    .max()
                    .unwrap_or(0)
            };
            let used = column_width("Node", |row| &row.0)
                + column_width("Shape", |row| &row.1)
                + column_width("Status", |row| &row.2)
                + 4 * 2
                + 5;
            let summary_width = terminal_width.saturating_sub(used).max(MIN_SUMMARY_WIDTH);
            for (node_label, shape_label, _, status_label, details) in rows {
                let (summary, complete) = summarize(&details, &node_label, &shape_label, summary_width);
                if !complete {
                    sections.push((format!("{node_label}@{shape_label} {status_label}"), details.clone()));
                }
                builder.push_record([node_label, shape_label, status_label, summary]);
            }
        } else {
            builder.push_record(["Node", "Shape", "Status"]);
            for (node_label, shape_label, _, status_label, _) in rows {
                builder.push_record([node_label, shape_label, status_label]);
            }
        }
        let mut table = builder.build();
        table.with(Style::modern_rounded());
        table.with(Modify::new(Segment::all()).with(Width::wrap(terminal_width)));
        writeln!(writer, "{table}")?;
        // Unlike the cells of the table, these lines are not wrapped, so a tree keeps its shape
        // and the ShExPaths in it can be copied whole.
        for (title, details) in sections {
            writeln!(writer)?;
            writeln!(writer, "{title}")?;
            for line in details.lines() {
                writeln!(writer, "  {line}")?;
            }
        }
        Ok(())
    }

    fn get_comparator(
        &self,
        sort_mode: &SortMode,
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

/// The fewest characters the summary column of a table with details is given, however narrow the
/// terminal is.
const MIN_SUMMARY_WIDTH: usize = 20;

/// The summary of `details` for the row of `node`@`shape`, cut with `…` to at most `width`
/// characters, and whether it is all of `details`.
///
/// The row already says the node and the shape, so `<node> does not conform to <shape>` is left
/// out of the summary; when that is all the first line says, the summary is the line below it.
fn summarize(details: &str, node: &str, shape: &str, width: usize) -> (String, bool) {
    let mut lines = details.trim_end().lines();
    let first = lines.next().unwrap_or("");
    let header = format!("{node} does not conform to {shape}");
    let (summary, rest) = match first.strip_prefix(&header) {
        Some("") => {
            let next = lines.next().unwrap_or("");
            let next = next.trim_start_matches(|c: char| "├└│─ ".contains(c));
            (next, true)
        },
        Some(cause) => (cause.trim_start_matches(':').trim_start(), lines.next().is_some()),
        None => (first, lines.next().is_some()),
    };
    if summary.chars().count() <= width {
        (summary.to_string(), !rest)
    } else {
        let cut: String = summary.chars().take(width.saturating_sub(1)).collect();
        (format!("{}…", cut.trim_end()), false)
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

/// Where a table with details writes the details of each result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DetailsLayout {
    /// A summary in the row, and the details that don't fit in it below the table, one line each,
    /// so a tree keeps its shape and the ShExPaths in it can be copied whole.
    #[default]
    Below,
    /// All the details in the row, wrapped to the width of the table.
    Table,
}

#[cfg(test)]
mod tests {
    use super::summarize;

    #[test]
    fn a_summary_leaves_out_the_node_and_the_shape() {
        let details = ":a does not conform to :S: property :p: expected exactly 1 value, found 0";
        assert_eq!(
            summarize(details, ":a", ":S", 80),
            ("property :p: expected exactly 1 value, found 0".to_string(), true)
        );
    }

    #[test]
    fn a_summary_of_a_tree_is_its_first_branch() {
        let details =
            ":a does not conform to :S\n└── property :p value :b does not conform to :T\n    └── property :q: ...";
        assert_eq!(
            summarize(details, ":a", ":S", 80),
            ("property :p value :b does not conform to :T".to_string(), false)
        );
    }

    #[test]
    fn a_long_summary_is_cut() {
        assert_eq!(
            summarize("Closed shape :S does not allow :z", ":a", ":S", 20),
            ("Closed shape :S doe…".to_string(), false)
        );
        assert_eq!(
            summarize("Shape passed :a@:S", ":a", ":S", 20),
            ("Shape passed :a@:S".to_string(), true)
        );
    }
}
