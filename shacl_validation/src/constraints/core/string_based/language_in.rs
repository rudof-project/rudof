use std::collections::HashSet;

use srdf::lang::Lang;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
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

impl<S> ConstraintComponent<S> for LanguageIn {
    fn evaluate(
        &self,
        _: &S,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            let literal = match node {
                Term::Literal(literal) => literal,
                _ => {
                    <LanguageIn as ConstraintComponent<S>>::make_validation_result(
                        self,
                        Some(node),
                        report,
                    );
                    break;
                }
            };
            if let Some(lang) = literal.lang() {
                if !self.langs.contains(lang) {
                    <LanguageIn as ConstraintComponent<S>>::make_validation_result(
                        self,
                        Some(node),
                        report,
                    );
                }
            }
        }
        Ok(())
    }
}
