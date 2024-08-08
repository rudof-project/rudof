use std::collections::HashSet;

use shacl_ast::Schema;
use srdf::lang::Lang;
use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::runner::ValidatorRunner;
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

impl<S: SRDFBasic> ConstraintComponent<S> for LanguageIn {
    fn evaluate(
        &self,
        _store: &S,
        _: &Schema,
        _: &dyn ValidatorRunner<S>,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for node in value_nodes {
            if let Some(literal) = S::term_as_literal(node) {
                if let Some(lang) = S::lang(&literal) {
                    if !self.langs.contains(&Lang::new(&lang)) {
                        ans = false;
                        report.make_validation_result(Some(node));
                    }
                }
            } else {
                ans = false;
                report.make_validation_result(Some(node))
            }
        }
        Ok(ans)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for LanguageIn {
    fn evaluate_default(
        &self,
        store: &S,
        schema: &Schema,
        runner: &DefaultValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(store, schema, runner, value_nodes, report)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for LanguageIn {
    fn evaluate_sparql(
        &self,
        store: &S,
        schema: &Schema,
        runner: &SparqlValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(store, schema, runner, value_nodes, report)
    }
}
