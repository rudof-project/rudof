use std::collections::HashSet;
use std::fmt;

use indoc::formatdoc;
use oxigraph::{model::Term, store::Store};

use crate::helper::sparql::{ask, select};

use super::{result::ValidationResult, validation_report_error::ValidationReportError};

pub struct ValidationReport {
    conforms: bool,
    result: Vec<ValidationResult>,
}

impl ValidationReport {
    pub fn new(conforms: bool, result: Vec<ValidationResult>) -> Self {
        ValidationReport { conforms, result }
    }

    pub fn add_result(&mut self, result: ValidationResult) {
        if self.conforms {
            // We add a result --> make the Report non-conformant
            self.conforms = false;
        }
        self.result.push(result)
    }

    pub fn parse(store: &Store, subject: &Term) -> Result<ValidationReport, ValidationReportError> {
        let conforms = Self::is_conforms(store, subject)?;

        let mut results = Vec::new();
        for _ in Self::get_results(store, subject)? {
            results.push(ValidationResult::parse(store, subject)?);
        }

        Ok(ValidationReport::new(conforms, results))
    }

    fn is_conforms(store: &Store, subject: &Term) -> Result<bool, ValidationReportError> {
        let query = formatdoc! {"
            ASK {{ {} {} true }}
        ", subject, shacl_ast::SH_CONFORMS.as_named_node()};
        Ok(ask(store, query)?)
    }

    fn get_results(store: &Store, subject: &Term) -> Result<HashSet<Term>, ValidationReportError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                {} {} ?this .
            }}
        ", subject, shacl_ast::SH_RESULT.as_named_node()};
        Ok(select(store, query)?)
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
        writeln!(f, "\tresult:")?;
        for result in &self.result {
            writeln!(f, "\t\t[")?;
            if let Some(term) = &result.focus_node() {
                writeln!(f, "\t\t\tfocus_node: {},", term)?;
            }
            if let Some(term) = &result.result_path() {
                writeln!(f, "\t\t\tresult_path: {},", term)?;
            }
            if let Some(term) = &result.result_severity() {
                writeln!(f, "\t\t\tresult_severity: {},", term)?;
            }
            if let Some(term) = &result.source_constraint() {
                writeln!(f, "\t\t\tsource_constraint: {},", term)?;
            }
            if let Some(term) = &result.source_constraint_component() {
                writeln!(f, "\t\t\tsource_constraint_component: {},", term)?;
            }
            if let Some(term) = &result.source_shape() {
                writeln!(f, "\t\t\tsource_shape: {},", term)?;
            }
            if let Some(term) = &result.value() {
                writeln!(f, "\t\t\tvalue: {},", term)?;
            }
            writeln!(f, "\t\t],")?;
        }
        writeln!(f, "]")
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self {
            conforms: true,
            result: Vec::new(),
        }
    }
}
