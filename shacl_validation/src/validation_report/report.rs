use std::fmt::{Debug, Display};

use colored::*;
use prefixmap::PrefixMap;
use srdf::{Object, Query, Rdf, SRDFBuilder};

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
    pub fn new() -> Self {
        Self::default()
    }

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
    pub fn parse<S: Query>(store: &S, subject: S::Term) -> Result<Self, ReportError> {
        let mut results = Vec::new();
        for result in get_objects_for(store, &subject, &shacl_ast::SH_RESULT.clone().into())? {
            results.push(ValidationResult::parse(store, &result)?);
        }
        Ok(ValidationReport::new().with_results(results))
    }

    pub fn conforms(&self) -> bool {
        self.results.is_empty()
    }

    pub fn to_rdf<RDF>(&self, rdf_writer: &mut RDF) -> Result<(), ReportError>
    where
        RDF: SRDFBuilder + Sized,
    {
        rdf_writer.add_prefix("sh", &shacl_ast::SH).map_err(|e| {
            ReportError::ValidationReportError {
                msg: format!("Error adding prefix to RDF: {e}"),
            }
        })?;
        let report_node: RDF::Subject = rdf_writer
            .add_bnode()
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error creating bnode: {e}"),
            })?
            .into();
        rdf_writer
            .add_type(report_node.clone(), shacl_ast::SH_VALIDATION_REPORT.clone())
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error type ValidationReport to bnode: {e}"),
            })?;

        let conforms: <RDF as Rdf>::IRI = shacl_ast::SH_CONFORMS.clone().into();
        let sh_result: <RDF as Rdf>::IRI = shacl_ast::SH_RESULT.clone().into();
        if self.results.is_empty() {
            let rdf_true: <RDF as Rdf>::Term = Object::boolean(true).into();
            rdf_writer
                .add_triple(report_node.clone(), conforms, rdf_true)
                .map_err(|e| ReportError::ValidationReportError {
                    msg: format!("Error adding conforms to bnode: {e}"),
                })?;
            return Ok(());
        } else {
            let rdf_false: <RDF as Rdf>::Term = Object::boolean(false).into();
            rdf_writer
                .add_triple(report_node.clone(), conforms, rdf_false)
                .map_err(|e| ReportError::ValidationReportError {
                    msg: format!("Error adding conforms to bnode: {e}"),
                })?;
            for result in self.results.iter() {
                let result_node: <RDF as Rdf>::BNode =
                    rdf_writer
                        .add_bnode()
                        .map_err(|e| ReportError::ValidationReportError {
                            msg: format!("Error creating bnode: {e}"),
                        })?;
                let result_node_term: <RDF as Rdf>::Term = result_node.into();
                rdf_writer
                    .add_triple(
                        report_node.clone(),
                        sh_result.clone(),
                        result_node_term.clone(),
                    )
                    .map_err(|e| ReportError::ValidationReportError {
                        msg: format!("Error adding conforms to bnode: {e}"),
                    })?;
                let result_node_subject: <RDF as Rdf>::Subject =
                    <RDF as Rdf>::Subject::try_from(result_node_term).map_err(|_e| {
                        ReportError::ValidationReportError {
                            msg: "Cannot convert subject to term".to_string(),
                        }
                    })?;
                result.to_rdf(rdf_writer, result_node_subject)?;
            }
        }
        Ok(())
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
    // TODO: This way to compare validation report results is wrong
    // Comparing only the len() is very weak
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
                    "Focus node {}, Component: {},{}{} severity: {}",
                    show_object(result.focus_node(), &self.nodes_prefixmap),
                    show_object(result.component(), &shacl_prefixmap),
                    show_object_opt("source shape", result.source(), &shacl_prefixmap),
                    show_object_opt("value", result.value(), &shacl_prefixmap),
                    show_object(result.severity(), &shacl_prefixmap)
                )?;
            }
            Ok(())
        }
    }
}

fn show_object(object: &Object, shacl_prefixmap: &PrefixMap) -> String {
    match object {
        Object::Iri(iri_s) => shacl_prefixmap.qualify(iri_s),
        Object::BlankNode(node) => format!("_:{node}"),
        Object::Literal(literal) => format!("{literal}"),
    }
}

fn show_object_opt(msg: &str, object: Option<&Object>, shacl_prefixmap: &PrefixMap) -> String {
    match object {
        None => String::new(),
        Some(Object::Iri(iri_s)) => shacl_prefixmap.qualify(iri_s),
        Some(Object::BlankNode(node)) => format!(" {msg}: _:{node}, "),
        Some(Object::Literal(literal)) => format!(" {msg}: {literal}, "),
    }
}
