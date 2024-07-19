use std::fmt;

use oxrdf::{Subject, Term};
use shacl_ast::SH_SOURCE_CONSTRAINT;
use shacl_ast::{
    SH_CONFORMS, SH_FOCUS_NODE, SH_RESULT, SH_RESULT_PATH, SH_RESULT_SEVERITY,
    SH_SOURCE_CONSTRAINT_COMPONENT, SH_SOURCE_SHAPE, SH_VALUE,
};

use srdf::SRDFGraph;

use crate::{get_triple_with_predicate, object, objects};

pub struct ValidationReport {
    conforms: bool,
    result: Vec<ValidationResult>,
}

impl ValidationReport {
    pub fn new(conforms: bool, result: Vec<ValidationResult>) -> Self {
        ValidationReport { conforms, result }
    }

    pub fn default() -> Self {
        ValidationReport {
            conforms: true,
            result: Vec::new(),
        }
    }

    pub fn parse(graph: SRDFGraph, subject: Subject) -> ValidationReport {
        let conforms: bool = match get_triple_with_predicate(&graph, &SH_CONFORMS).obj() {
            Term::NamedNode(_) => todo!(),
            Term::BlankNode(_) => todo!(),
            Term::Literal(literal) => match literal.destruct().0.parse() {
                Ok(conforms) => conforms,
                Err(_) => todo!(),
            },
        };

        let results = objects(&graph, &subject, &SH_RESULT);
        let mut result = Vec::new();
        for _ in results {
            result.push(ValidationResult::parse(&graph, &subject))
        }

        ValidationReport::new(conforms, result)
    }

    pub fn set_non_conformant(&mut self) {
        self.conforms = false;
    }

    pub fn extend_result(&mut self, result: Vec<ValidationResult>) {
        self.result.extend(result)
    }
}

impl PartialEq for ValidationReport {
    fn eq(&self, other: &Self) -> bool {
        if self.conforms != other.conforms {
            return false;
        }
        if self.result.len() != other.result.len() {
            return false;
        }

        true
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Validation Report: [")?;
        writeln!(f, "\tconforms: {},", self.conforms)?;
        writeln!(f, "\tresult: {{")?;
        for result in &self.result {
            writeln!(f, "\t\t[")?;
            if let Some(focus_node) = &result.focus_node {
                writeln!(f, "\t\t\tfocus_node: {},", focus_node)?;
            }
            if let Some(result_path) = &result.result_path {
                writeln!(f, "\t\t\tresult_path: {},", result_path)?;
            }
            if let Some(result_severity) = &result.result_severity {
                writeln!(f, "\t\t\tresult_severity: {},", result_severity)?;
            }
            if let Some(source_constraint) = &result.source_constraint {
                writeln!(f, "\t\t\tsource_constraint: {},", source_constraint)?;
            }
            if let Some(source_constraint_component) = &result.source_constraint_component {
                writeln!(
                    f,
                    "\t\t\tsource_constraint_component: {},",
                    source_constraint_component
                )?;
            }
            if let Some(source_shape) = &result.source_shape {
                writeln!(f, "\t\t\tsource_shape: {},", source_shape)?;
            }
            if let Some(value) = &result.value {
                writeln!(f, "\t\t\tvalue: {},", value)?;
            }
            writeln!(f, "\t\t],")?;
        }
        writeln!(f, "]")
    }
}

pub struct ValidationResult {
    focus_node: Option<Term>,
    result_severity: Option<Term>,
    result_path: Option<Term>,
    source_constraint: Option<Term>,
    source_constraint_component: Option<Term>,
    source_shape: Option<Term>,
    value: Option<Term>,
}

impl ValidationResult {
    pub fn new(
        focus_node: Option<Term>,
        result_severity: Option<Term>,
        result_path: Option<Term>,
        source_constraint: Option<Term>,
        source_constraint_component: Option<Term>,
        source_shape: Option<Term>,
        value: Option<Term>,
    ) -> Self {
        ValidationResult {
            focus_node,
            result_severity,
            result_path,
            source_constraint,
            source_constraint_component,
            source_shape,
            value,
        }
    }

    fn parse(graph: &SRDFGraph, subject: &Subject) -> ValidationResult {
        let focus_node = object(graph, subject, &SH_FOCUS_NODE);
        let result_severity = object(graph, subject, &SH_RESULT_SEVERITY);
        let result_path = object(graph, subject, &SH_RESULT_PATH);
        let source_constraint = object(graph, subject, &SH_SOURCE_CONSTRAINT);
        let source_constraint_component = object(graph, subject, &SH_SOURCE_CONSTRAINT_COMPONENT);
        let source_shape = object(graph, subject, &SH_SOURCE_SHAPE);
        let value = object(graph, subject, &SH_VALUE);

        ValidationResult::new(
            focus_node,
            result_severity,
            result_path,
            source_constraint,
            source_constraint_component,
            source_shape,
            value,
        )
    }
}
