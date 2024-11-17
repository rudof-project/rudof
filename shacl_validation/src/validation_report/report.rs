use std::fmt::Debug;
use std::fmt::Display;

use colored::*;
use prefixmap::PrefixMap;
use shacl_ast::vocab::SH_RESULT;
use srdf::model::rdf::TObject;
use srdf::model::rdf::TPredicate;
use srdf::model::rdf::Rdf;
use srdf::model::Iri;
use srdf::model::Term;

use crate::helpers::srdf::get_objects_for;

use super::result::ValidationResult;
use super::validation_report_error::ReportError;

#[derive(Debug, Clone)]
pub struct ValidationReport<R: Rdf> {
    results: Vec<ValidationResult<R>>,
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,
    ok_color: Option<Color>,
    fail_color: Option<Color>,
    display_with_colors: bool,
}

impl<R: Rdf> ValidationReport<R> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_results(mut self, results: Vec<ValidationResult<R>>) -> Self {
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

    pub fn results(&self) -> &Vec<ValidationResult<R>> {
        &self.results
    }

    pub fn parse(store: &R, subject: TObject<R>) -> Result<Self, ReportError> {
        let mut results = Vec::new();
        for result in get_objects_for(
            store,
            &subject,
            &TPredicate::<R>::new(SH_RESULT.as_str()).into(),
        )? {
            results.push(ValidationResult::parse(store, &result)?);
        }
        Ok(ValidationReport::new().with_results(results))
    }

    pub fn conforms(&self) -> bool {
        self.results.is_empty()
    }
}

impl<R: Rdf> Default for ValidationReport<R> {
    fn default() -> Self {
        ValidationReport {
            results: Vec::new(),
            nodes_prefixmap: PrefixMap::default(),
            shapes_prefixmap: PrefixMap::default(),
            ok_color: Some(Color::Green),
            fail_color: Some(Color::Red),
            display_with_colors: true,
        }
    }
}

impl<R: Rdf> PartialEq for ValidationReport<R> {
    // TODO: Are we sure that this way to compare validation report results is OK?
    // Comparing only the len() seems weak??
    fn eq(&self, other: &Self) -> bool {
        if self.results.len() != other.results.len() {
            return false;
        }
        true
    }
}

impl<R: Rdf> Display for ValidationReport<R> {
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
                    show_node::<R>(result.focus_node(), &self.nodes_prefixmap),
                    show_component::<R>(result.component(), &shacl_prefixmap),
                    show_severity::<R>(result.severity(), &shacl_prefixmap)
                )?;
            }
            Ok(())
        }
    }
}

fn show_node<R: Rdf>(node: &TObject<R>, prefixmap: &PrefixMap) -> String {
    match (node.is_iri(), node.is_blank_node(), node.is_literal()) {
        (true, false, false) => prefixmap.qualify(&node.as_iri().unwrap().as_iri_s()),
        (false, true, false) => format!("_:{}", node.as_blank_node().unwrap().to_string()),
        (false, false, true) => format!("{}", node.as_literal().unwrap()),
        _ => unreachable!(),
    }
}

fn show_component<R: Rdf>(component: &TObject<R>, shacl_prefixmap: &PrefixMap) -> String {
    match (
        component.is_iri(),
        component.is_blank_node(),
        component.is_literal(),
    ) {
        (true, false, false) => shacl_prefixmap.qualify(&component.as_iri().unwrap().as_iri_s()),
        (false, true, false) => format!("_:{}", component.as_blank_node().unwrap().to_string()),
        (false, false, true) => format!("{}", component.as_literal().unwrap()),
        _ => unreachable!(),
    }
}

fn show_severity<R: Rdf>(severity: &TObject<R>, shacl_prefixmap: &PrefixMap) -> String {
    match (
        severity.is_iri(),
        severity.is_blank_node(),
        severity.is_literal(),
    ) {
        (true, false, false) => shacl_prefixmap.qualify(&severity.as_iri().unwrap().as_iri_s()),
        (false, true, false) => format!("_:{}", severity.as_blank_node().unwrap().to_string()),
        (false, false, true) => format!("{}", severity.as_literal().unwrap()),
        _ => unreachable!(),
    }
}
