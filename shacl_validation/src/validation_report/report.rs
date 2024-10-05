use std::fmt::Debug;

use srdf::SRDFBasic;
use srdf::SRDF;

use crate::helper::srdf::get_objects_for;

use super::result::ValidationResult;
use super::validation_report_error::ReportError;

pub struct ValidationReport<S: SRDFBasic> {
    results: Vec<ValidationResult<S>>,
}

impl<S: SRDFBasic> ValidationReport<S> {
    pub fn new(results: Vec<ValidationResult<S>>) -> Self {
        Self { results }
    }

    pub fn results(&self) -> &Vec<ValidationResult<S>> {
        &self.results
    }
}

impl<S: SRDF> ValidationReport<S> {
    pub fn parse(store: &S, subject: S::Term) -> Result<Self, ReportError> {
        let mut results = Vec::new();
        for result in get_objects_for(store, &subject, &S::iri_s2iri(&shacl_ast::SH_RESULT))? {
            results.push(ValidationResult::parse(store, &result)?);
        }
        Ok(ValidationReport::new(results))
    }
}

impl<S: SRDFBasic> Debug for ValidationReport<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationReport")
            .field("results", &self.results)
            .finish()
    }
}

impl<S: SRDFBasic> Default for ValidationReport<S> {
    fn default() -> Self {
        ValidationReport {
            results: Vec::new(),
        }
    }
}

impl<S: SRDFBasic> PartialEq for ValidationReport<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.results.len() != other.results.len() {
            return false;
        }
        true
    }
}
