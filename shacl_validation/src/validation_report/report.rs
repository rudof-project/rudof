use std::fmt;

use srdf::SRDFBasic;
use srdf::SRDF;

use crate::helper::srdf::get_object_for;
use crate::helper::srdf::get_objects_for;

use super::result::ValidationResult;
use super::validation_report_error::ValidationReportError;

#[derive(Default)]
pub struct ValidationReport<S: SRDFBasic> {
    conforms: bool,
    results: Vec<ValidationResult<S>>,
}

impl<S: SRDF + SRDFBasic> ValidationReport<S> {
    pub(crate) fn default() -> Self {
        ValidationReport {
            conforms: true,
            results: Vec::new(),
        }
    }

    fn new(conforms: bool, results: Vec<ValidationResult<S>>) -> Self {
        ValidationReport { conforms, results }
    }

    fn is_conforms(self, store: &S, subject: &S::Term) -> Result<bool, ValidationReportError> {
        let predicate = S::iri_s2iri(&shacl_ast::SH_CONFORMS);
        if let Some(term) = get_object_for(store, &subject, &predicate)? {
            if let Some(is_conforms) = S::term_as_boolean(&term) {
                Ok(is_conforms)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub(crate) fn add_result(&mut self, result: ValidationResult<S>) {
        // We add a result --> make the Report non-conformant
        if self.conforms {
            self.conforms = false;
        }
        self.results.push(result)
    }

    pub fn parse(store: &S, subject: S::Term) -> Result<Self, ValidationReportError> {
        let mut report = ValidationReport::<S>::default();

        for result in get_objects_for(store, &subject, &S::iri_s2iri(&shacl_ast::SH_RESULT))? {
            report.add_result(ValidationResult::parse(store, &result)?);
        }
        Ok(report)
    }
}

impl<S: SRDF + SRDFBasic> PartialEq for ValidationReport<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.conforms != other.conforms {
            return false;
        }
        if self.results.len() != other.results.len() {
            return false;
        }
        true
    }
}

impl<S: SRDF + SRDFBasic> fmt::Display for ValidationReport<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Validation Report: [")?;
        writeln!(f, "\tconforms: {},", self.conforms)?;
        writeln!(f, "\tresult:")?;
        for result in &self.results {
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
