use std::fmt;

use srdf::SRDFBasic;
use srdf::SRDF;

use crate::helper::srdf::get_object_for;
use crate::helper::srdf::get_objects_for;
use crate::helper::term::Term;

use super::result::ValidationResult;
use super::validation_report_error::ValidationReportError;

#[derive(Default)]
pub struct ValidationReport<'a> {
    conforms: bool,
    results: Vec<ValidationResult<'a>>,
}

impl<'a> ValidationReport<'a> {
    fn new(conforms: bool, results: Vec<ValidationResult>) -> Self {
        ValidationReport { conforms, results }
    }

    fn is_conforms<S: SRDF + SRDFBasic + Default>(
        self,
        store: &S,
        subject: &Term,
    ) -> Result<bool, ValidationReportError> {
        let subject = match subject {
            Term::IRI(_) => subject,
            Term::BlankNode(_) => subject,
            Term::Literal(_) => return Err(ValidationReportError::LiteralToSubject),
        };

        if let Some(term) = get_object_for(store, &subject, &S::iri_s2iri(&shacl_ast::SH_CONFORMS))?
        {
            match &term {
                Term::IRI(_) => Err(ValidationReportError::InvalidTerm),
                Term::BlankNode(_) => Err(ValidationReportError::InvalidTerm),
                Term::Literal(content) => todo!(),
            }
        } else {
            Ok(false)
        }
    }

    pub(crate) fn add_result(&mut self, result: ValidationResult) {
        // We add a result --> make the Report non-conformant
        if self.conforms {
            self.conforms = false;
        }
        self.results.push(result)
    }

    fn parse<S: SRDF + SRDFBasic + Default>(
        self,
        store: &S,
        subject: &Term,
    ) -> Result<Self, ValidationReportError>
    where
        Self: Sized,
    {
        let mut report = ValidationReport::default();

        for result in get_objects_for(store, &subject, &S::iri_s2iri(&shacl_ast::SH_RESULT))? {
            report.add_result(ValidationResult::parse(store, &result)?);
        }
        Ok(report)
    }
}

impl<'a> PartialEq for ValidationReport<'a> {
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

impl<'a> fmt::Display for ValidationReport<'a> {
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
