use oxrdf::{Subject, Term};
use shacl_ast::{SH_CONFORMS, SH_RESULT};
use srdf::{SRDFGraph, SRDF};
use std::collections::HashSet;
use std::fmt;

use crate::shacl_validation_vocab::SHT_FAILURE;

use super::{result::ValidationResult, validation_report_error::ValidationReportError};

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

    fn is_conforms(graph: &SRDFGraph, subject: &Subject) -> Result<bool, ValidationReportError> {
        match graph.objects_for_subject_predicate(&subject, &SH_CONFORMS.as_named_node()) {
            Ok(objects) => match objects.into_iter().nth(0) {
                Some(object) => match object {
                    Term::NamedNode(_) => todo!(),
                    Term::BlankNode(_) => todo!(),
                    Term::Literal(literal) => match literal.destruct().0.parse() {
                        Ok(conforms) => Ok(conforms),
                        Err(_) => todo!(),
                    },
                },
                None => todo!(),
            },
            Err(_) => todo!(),
        }
    }

    fn get_results(
        graph: &SRDFGraph,
        subject: &Subject,
    ) -> Result<HashSet<Term>, ValidationReportError> {
        match graph.objects_for_subject_predicate(subject, &SH_RESULT.as_named_node()) {
            Ok(objects) => Ok(objects),
            Err(_) => todo!(),
        }
    }

    pub fn parse(
        graph: SRDFGraph,
        subject: Subject,
    ) -> Result<ValidationReport, ValidationReportError> {
        let mut results = Vec::new();

        let conforms = match subject {
            Subject::NamedNode(named_node) => {
                if &named_node == SHT_FAILURE.as_named_node() {
                    false
                } else {
                    todo!()
                }
            }
            Subject::BlankNode(_) => {
                for _ in 0..Self::get_results(&graph, &subject)?.len() {
                    match ValidationResult::parse(&graph, &subject) {
                        Ok(result) => results.push(result),
                        Err(_) => todo!(),
                    }
                }
                Self::is_conforms(&graph, &subject)?
            }
        };

        Ok(ValidationReport::new(conforms, results))
    }

    pub fn set_non_conformant(&mut self) {
        self.conforms = false;
    }

    pub fn add_result(&mut self, result: ValidationResult) {
        self.result.push(result)
    }

    pub fn extend_results(&mut self, results: Vec<ValidationResult>) {
        self.result.extend(results)
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
            if let Some(focus_node) = &result.focus_node() {
                writeln!(f, "\t\t\tfocus_node: {},", focus_node)?;
            }
            if let Some(result_path) = &result.result_path() {
                writeln!(f, "\t\t\tresult_path: {},", result_path)?;
            }
            if let Some(result_severity) = &result.result_severity() {
                writeln!(f, "\t\t\tresult_severity: {},", result_severity)?;
            }
            if let Some(source_constraint) = &result.source_constraint() {
                writeln!(f, "\t\t\tsource_constraint: {},", source_constraint)?;
            }
            if let Some(source_constraint_component) = &result.source_constraint_component() {
                writeln!(
                    f,
                    "\t\t\tsource_constraint_component: {},",
                    source_constraint_component
                )?;
            }
            if let Some(source_shape) = &result.source_shape() {
                writeln!(f, "\t\t\tsource_shape: {},", source_shape)?;
            }
            if let Some(value) = &result.value() {
                writeln!(f, "\t\t\tvalue: {},", value)?;
            }
            writeln!(f, "\t\t],")?;
        }
        writeln!(f, "]")
    }
}
