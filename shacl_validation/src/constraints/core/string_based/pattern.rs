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

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Pattern {
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
                    let query = match &self.flags {
                        Some(flags) => formatdoc! {
                            "ASK {{ FILTER (regex(str({}), {}, {})) }}",
                            value_node, self.pattern, flags
                        },
                        None => formatdoc! {
                            "ASK {{ FILTER (regex(str({}), {})) }}",
                            value_node, self.pattern
                        },
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
