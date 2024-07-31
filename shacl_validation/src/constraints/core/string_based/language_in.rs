use std::collections::HashSet;

use srdf::lang::Lang;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// The condition specified by sh:languageIn is that the allowed language tags
/// for each value node are limited by a given list of language tags.
///
/// https://www.w3.org/TR/shacl/#LanguageInConstraintComponent
pub(crate) struct LanguageIn {
    langs: Vec<Lang>,
}

impl LanguageIn {
    pub fn new(langs: Vec<Lang>) -> Self {
        LanguageIn { langs }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for LanguageIn {
    fn evaluate(
        &self,
        _: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if let Some(literal) = S::term_as_literal(node) {
                if let Some(lang) = S::lang(&literal) {
                    if !self.langs.contains(&Lang::new(&lang)) {
                        self.make_validation_result(Some(node), report);
                    }
                }
            } else {
                self.make_validation_result(Some(node), report)
            }
        }
        Ok(())
    }
}
