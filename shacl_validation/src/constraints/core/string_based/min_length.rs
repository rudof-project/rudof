use std::collections::HashSet;

use indoc::formatdoc;
use srdf::{QuerySRDF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:minLength specifies the minimum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MinLengthConstraintComponent
pub(crate) struct MinLength {
    min_length: isize,
}

impl MinLength {
    pub fn new(min_length: isize) -> Self {
        MinLength { min_length }
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for MinLength {
    fn evaluate_default(
        &self,
        _: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if S::term_is_bnode(node) {
                report.make_validation_result(Some(node));
            } else {
                return Err(ConstraintError::NotImplemented);
            }
        }
        Ok(())
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for MinLength {
    fn evaluate_sparql(
        &self,
        store: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if S::term_is_bnode(node) {
                report.make_validation_result(Some(node));
            } else {
                let query = formatdoc! {
                    " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                    node, self.min_length
                };
                let ans = match store.query_ask(&query) {
                    Ok(ans) => ans,
                    Err(_) => return Err(ConstraintError::Query),
                };
                if !ans {
                    report.make_validation_result(Some(node));
                }
            }
        }
        Ok(())
    }
}
