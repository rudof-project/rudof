use super::result::ValidationResult;
use super::validation_report_error::ReportError;
use colored::*;
use iri_s::IriS;
use prefixmap::PrefixMap;
use shacl_ast::shacl_vocab::{sh, sh_conforms, sh_result, sh_validation_report};
use shacl_ir::severity::CompiledSeverity;
use srdf::{BuildRDF, FocusRDF, IriOrBlankNode, Object, Rdf, SHACLPath};
use std::{
    fmt::{Debug, Display},
    io::{Error, Write},
};
use tabled::{
    builder::Builder,
    settings::{Modify, Style, Width, object::Segment},
};

#[derive(Debug, Clone)]
pub struct ValidationReport {
    results: Vec<ValidationResult>,
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,
    ok_color: Option<Color>,
    info_color: Option<Color>,
    warning_color: Option<Color>,
    debug_color: Option<Color>,
    trace_color: Option<Color>,
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
    pub fn parse<S: FocusRDF>(store: &mut S, subject: S::Term) -> Result<Self, ReportError> {
        let mut results = Vec::new();
        for result in store
            .objects_for(&subject, &sh_result().clone().into())
            .map_err(|e| ReportError::ObjectsFor {
                subject: subject.to_string(),
                predicate: sh_result().to_string(),
                error: e.to_string(),
            })?
        {
            results.push(ValidationResult::parse(store, &result)?);
        }
        Ok(ValidationReport::new().with_results(results))
    }

    pub fn conforms(&self) -> bool {
        self.results.is_empty()
    }

    pub fn count_violations(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity() == &CompiledSeverity::Violation)
            .count()
    }

    pub fn count_warnings(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity() == &CompiledSeverity::Warning)
            .count()
    }

    pub fn to_rdf<RDF>(&self, rdf_writer: &mut RDF) -> Result<(), ReportError>
    where
        RDF: BuildRDF + Sized,
    {
        rdf_writer
            .add_prefix("sh", sh())
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error adding prefix to RDF: {e}"),
            })?;
        let report_node: RDF::Subject = rdf_writer
            .add_bnode()
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error creating bnode: {e}"),
            })?
            .into();
        rdf_writer
            .add_type(report_node.clone(), sh_validation_report().clone())
            .map_err(|e| ReportError::ValidationReportError {
                msg: format!("Error type ValidationReport to bnode: {e}"),
            })?;

        let conforms: <RDF as Rdf>::IRI = sh_conforms().clone().into();
        let sh_result: <RDF as Rdf>::IRI = sh_result().clone().into();
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

    pub fn show_as_table(
        &self,
        mut writer: Box<dyn Write + 'static>,
        _sort_mode: SortModeReport,
        with_details: bool,
        terminal_width: usize,
    ) -> Result<(), Error> {
        let mut builder = Builder::default();
        if with_details {
            builder.push_record([
                "Severity",
                "Node",
                "Component",
                "Path",
                "Value",
                "Source shape",
                "Details",
            ]);
        } else {
            builder.push_record([
                "Severity",
                "node",
                "Component",
                "Path",
                "value",
                "Source shape",
            ]);
        }
        if self.results.is_empty() {
            let str = "No Errors found";
            if self.display_with_colors {
                if let Some(ok_color) = self.ok_color {
                    write!(writer, "{}", str.color(ok_color))?;
                } else {
                    write!(writer, "{str}")?;
                }
            } else {
                write!(writer, "{str}")?;
            }
            Ok(())
        } else {
            let shacl_prefixmap = if self.display_with_colors {
                PrefixMap::basic()
            } else {
                PrefixMap::basic()
                    .with_hyperlink(true)
                    .without_default_colors()
            };
            for result in self.results.iter() {
                let severity_str = show_severity(result.severity(), &shacl_prefixmap);
                let severity = if self.display_with_colors {
                    let color = calculate_color(result.severity(), self);
                    severity_str.color(color)
                } else {
                    ColoredString::from(severity_str)
                };
                let node = show_object(result.focus_node(), &self.nodes_prefixmap);
                let component = show_object(result.component(), &shacl_prefixmap);
                let path = show_path_opt(result.path(), &self.shapes_prefixmap);
                let source = show_object_opt(result.source(), &self.shapes_prefixmap);
                let value = show_object_opt(result.value(), &self.nodes_prefixmap);
                let details = result.message().unwrap_or("").to_string();
                if with_details {
                    builder.push_record([
                        &severity.to_string(),
                        &node,
                        &component,
                        &path,
                        &value,
                        &source,
                        &details,
                    ]);
                } else {
                    builder.push_record([
                        &severity.to_string(),
                        &node,
                        &component,
                        &path,
                        &value,
                        &source,
                    ]);
                }
            }
            let mut table = builder.build();
            table.with(Style::modern_rounded());
            table.with(Modify::new(Segment::all()).with(Width::wrap(terminal_width)));
            writeln!(writer, "{table}")?;
            Ok(())
        }
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
            info_color: Some(Color::Blue),
            warning_color: Some(Color::Yellow),
            debug_color: Some(Color::Magenta),
            trace_color: Some(Color::Cyan),
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
                let severity_str = show_severity(result.severity(), &shacl_prefixmap);
                if self.display_with_colors {
                    let color = calculate_color(result.severity(), self);
                    write!(f, "{}", severity_str.color(color))?;
                } else {
                    writeln!(f, "{severity_str}")?;
                };
                let msg = format!(
                    " node: {} {}\n{}{}{}{}",
                    show_object(result.focus_node(), &self.nodes_prefixmap),
                    show_object(result.component(), &shacl_prefixmap),
                    result.message().unwrap_or(""),
                    show_path_opt(result.path(), &self.shapes_prefixmap),
                    show_object_opt(result.source(), &self.shapes_prefixmap),
                    show_object_opt(result.value(), &self.nodes_prefixmap)
                );
                writeln!(f, "{msg}")?;
            }
            Ok(())
        }
    }
}

fn show_severity(severity: &CompiledSeverity, shacl_prefixmap: &PrefixMap) -> String {
    shacl_prefixmap.qualify(&severity.to_iri())
}

fn show_object(object: &Object, shacl_prefixmap: &PrefixMap) -> String {
    match object {
        Object::Iri(iri_s) => shacl_prefixmap.qualify(iri_s),
        Object::BlankNode(node) => format!("_:{node}"),
        Object::Literal(literal) => format!("{literal}"),
        Object::Triple { .. } => todo!(),
    }
}

fn show_iri(iri: &IriS, prefixmap: &PrefixMap) -> String {
    prefixmap.qualify(iri)
}

fn show_subject(subject: &IriOrBlankNode, prefixmap: &PrefixMap) -> String {
    match subject {
        IriOrBlankNode::Iri(iri_s) => prefixmap.qualify(iri_s),
        IriOrBlankNode::BlankNode(node) => format!("_:{node}"),
    }
}

fn show_object_opt(object: Option<&Object>, shacl_prefixmap: &PrefixMap) -> String {
    match object {
        None => String::new(),
        Some(Object::Iri(iri_s)) => shacl_prefixmap.qualify(iri_s),
        Some(Object::BlankNode(node)) => format!("_:{node}"),
        Some(Object::Literal(literal)) => format!("{literal}"),
        Some(Object::Triple {
            subject,
            predicate,
            object,
        }) => format!(
            "<<{} {} {}>>",
            show_subject(subject, shacl_prefixmap),
            show_iri(predicate, shacl_prefixmap),
            show_object(object, shacl_prefixmap)
        ),
    }
}

fn show_path_opt(object: Option<&SHACLPath>, shacl_prefixmap: &PrefixMap) -> String {
    match object {
        None => String::new(),
        Some(SHACLPath::Predicate { pred }) => {
            let path = shacl_prefixmap.qualify(pred);
            format!("{path}")
        }
        Some(path) => format!("{path}"),
    }
}

fn calculate_color(severity: &CompiledSeverity, report: &ValidationReport) -> Color {
    match severity {
        CompiledSeverity::Violation => report.fail_color.unwrap_or(Color::Red),
        CompiledSeverity::Info => report.info_color.unwrap_or(Color::Blue),
        CompiledSeverity::Warning => report.warning_color.unwrap_or(Color::Yellow),
        CompiledSeverity::Debug => report.debug_color.unwrap_or(Color::Magenta),
        CompiledSeverity::Trace => report.trace_color.unwrap_or(Color::Cyan),
        CompiledSeverity::Generic(_) => Color::White,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SortModeReport {
    #[default]
    Node,
    Severity,
    Shape,
    Component,
    Source,
    Path,
    Value,
    Details,
}
