use std::collections::HashSet;

use indoc::formatdoc;
use srdf::{QuerySRDF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
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

impl<S: SRDF> DefaultConstraintComponent<S> for Pattern {
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

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Pattern {
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
