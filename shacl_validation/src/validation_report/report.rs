use std::fmt::{Debug, Display};

use colored::*;
use prefixmap::PrefixMap;
use srdf::{Object, Query};

use crate::helpers::srdf::get_objects_for;

use super::result::ValidationResult;
use super::validation_report_error::ReportError;

#[derive(Debug, Clone)]
pub struct ValidationReport {
    results: Vec<ValidationResult>,
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,
    ok_color: Option<Color>,
    fail_color: Option<Color>,
    display_with_colors: bool,
}

impl ValidationReport {
    pub fn with_results(mut self, results: Vec<ValidationResult>) -> Self {
        self.results = results;
        self
    }

    /// Sets the same prefixmap for nodes and shapes
    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.nodes_prefixmap = prefixmap.clone();
        self.shapes_prefixmap = prefixmap;
        self
    }

    /// Sets the prefixmap for nodes
    pub fn with_nodes_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.nodes_prefixmap = prefixmap;
        self
    }

    /// Sets the prefixmap for shapes
    pub fn with_shapes_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.shapes_prefixmap = prefixmap;
        self
    }

    pub fn without_colors(mut self) -> Self {
        self.ok_color = None;
        self.fail_color = None;
        self
    }

    pub fn with_ok_color(mut self, color: Color) -> Self {
        self.ok_color = Some(color);
        self
    }

    pub fn with_fail_color(mut self, color: Color) -> Self {
        self.fail_color = Some(color);
        self
    }

    pub fn results(&self) -> &Vec<ValidationResult> {
        &self.results
    }
}

impl ValidationReport {
    pub fn parse<Q: Query>(store: &Q, subject: Q::Term) -> Result<Self, ReportError> {
        let mut results = Vec::new();
        for result in get_objects_for(store, &subject, &shacl_ast::SH_RESULT.clone().into())? {
            results.push(ValidationResult::parse(store, &result)?);
        }
        Ok(ValidationReport::default().with_results(results))
    }

    pub fn conforms(&self) -> bool {
        self.results.is_empty()
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        ValidationReport {
            results: Vec::new(),
            nodes_prefixmap: PrefixMap::new(),
            shapes_prefixmap: PrefixMap::new(),
            ok_color: Some(Color::Green),
            fail_color: Some(Color::Red),
            display_with_colors: true,
        }
    }
}

impl PartialEq for ValidationReport {
    // TODO: Are we sure that this way to compare validation report results is OK?
    // Comparing only the len() seems weak??
    fn eq(&self, other: &Self) -> bool {
        if self.results.len() != other.results.len() {
            return false;
        }
        true
    }
}

impl Display for ValidationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.results.is_empty() {
            let str = "No Errors found";
            if self.display_with_colors {
                if let Some(ok_color) = self.ok_color {
                    write!(f, "{}", str.color(ok_color))?;
                } else {
                    write!(f, "{str}")?;
                }
            } else {
                write!(f, "{str}")?;
            }
            Ok(())
        } else {
            let str = format!("{} errors found", self.results.len());
            if self.display_with_colors {
                if let Some(fail_color) = self.fail_color {
                    writeln!(f, "{}", str.color(fail_color))?;
                } else {
                    writeln!(f, "{str}")?;
                }
            } else {
                writeln!(f, "{str}")?;
            };
            let shacl_prefixmap = if self.display_with_colors {
                PrefixMap::basic()
            } else {
                PrefixMap::basic()
                    .with_hyperlink(true)
                    .without_default_colors()
            };
            for result in self.results.iter() {
                writeln!(
                    f,
                    "Focus node {}, Component: {}, severity: {}",
                    show_node(result.focus_node(), &self.nodes_prefixmap),
                    show_component(result.component(), &shacl_prefixmap),
                    show_severity(result.severity(), &shacl_prefixmap)
                )?;
            }
            Ok(())
        }
    }
}

fn show_node(node: &Object, prefixmap: &PrefixMap) -> String {
    match node {
        Object::Iri(iri_s) => prefixmap.qualify(iri_s),
        Object::BlankNode(node) => format!("_:{node}"),
        Object::Literal(literal) => format!("{literal}"),
    }
}

fn show_component(component: &Object, shacl_prefixmap: &PrefixMap) -> String {
    match component {
        Object::Iri(iri_s) => shacl_prefixmap.qualify(iri_s),
        Object::BlankNode(node) => format!("_:{node}"),
        Object::Literal(literal) => format!("{literal}"),
    }
}

fn show_severity(severity: &Object, shacl_prefixmap: &PrefixMap) -> String {
    match severity {
        Object::Iri(iri_s) => shacl_prefixmap.qualify(iri_s),
        Object::BlankNode(node) => format!("_:{node}"),
        Object::Literal(literal) => format!("{literal}"),
    }
}
