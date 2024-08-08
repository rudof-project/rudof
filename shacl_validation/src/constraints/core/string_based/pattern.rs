use std::collections::HashSet;

use indoc::formatdoc;
use shacl_ast::Schema;
use srdf::{QuerySRDF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::validation_report::report::ValidationReport;

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
pub(crate) struct Pattern {
    pattern: String,
    flags: Option<String>,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Self {
        Pattern { pattern, flags }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Pattern {
    fn evaluate_default(
        &self,
        _store: &S,
        _: &Schema,
        _: &DefaultValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for node in value_nodes {
            if S::term_is_bnode(node) {
                ans = false;
                report.make_validation_result(Some(node));
            } else {
                return Err(ConstraintError::NotImplemented);
            }
        }
        Ok(ans)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Pattern {
    fn evaluate_sparql(
        &self,
        store: &S,
        _: &Schema,
        _: &SparqlValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for node in value_nodes {
            if S::term_is_bnode(node) {
                ans = false;
                report.make_validation_result(Some(node));
            } else {
                let query = match &self.flags {
                    Some(flags) => formatdoc! {
                        "ASK {{ FILTER (regex(str({}), {}, {})) }}",
                        node, self.pattern, flags
                    },
                    None => formatdoc! {
                        "ASK {{ FILTER (regex(str({}), {})) }}",
                        node, self.pattern
                    },
                };
                let ask = match store.query_ask(&query) {
                    Ok(ask) => ask,
                    Err(_) => return Err(ConstraintError::Query),
                };
                if !ask {
                    ans = false;
                    report.make_validation_result(Some(node));
                }
            }
        }
        Ok(ans)
    }
}
