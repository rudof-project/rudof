use indoc::formatdoc;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::shape::ValueNode;
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

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for MinLength {
    fn evaluate_default(
        &self,
        _executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                if S::term_is_bnode(value_node) {
                    ans = false;
                    report.make_validation_result(focus_node, context, Some(value_node));
                } else {
                    return Err(ConstraintError::NotImplemented);
                }
            }
        }
        Ok(ans)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for MinLength {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                if S::term_is_bnode(value_node) {
                    ans = false;
                    report.make_validation_result(focus_node, context, Some(value_node));
                } else {
                    let query = formatdoc! {
                        " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                        value_node, self.min_length
                    };
                    let ask = match executor.store().query_ask(&query) {
                        Ok(ask) => ask,
                        Err(_) => return Err(ConstraintError::Query),
                    };
                    if !ask {
                        ans = false;
                        report.make_validation_result(focus_node, context, Some(value_node));
                    }
                }
            }
        }
        Ok(ans)
    }
}
