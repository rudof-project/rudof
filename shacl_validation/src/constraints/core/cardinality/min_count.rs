use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::shape::ValueNode;
use crate::validation_report::report::ValidationReport;

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// - IRI: https://www.w3.org/TR/shacl/#MinCountConstraintComponent
/// - DEF: If the number of value nodes is less than $minCount, there is a
///   validation result.
pub(crate) struct MinCount {
    min_count: isize,
}

impl MinCount {
    pub fn new(min_count: isize) -> Self {
        MinCount { min_count }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for MinCount {
    fn evaluate(
        &self,
        _: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        if self.min_count == 0 {
            // If min_count is 0, then it always passes
            return Ok(true);
        }
        let mut ans = true;
        for (focus_node, value_nodes) in value_nodes {
            if (value_nodes.len() as isize) < self.min_count {
                ans = false;
                report.make_validation_result(focus_node, context, None);
            }
        }
        Ok(ans)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for MinCount {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(executor, context, value_nodes, report)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for MinCount {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(executor, context, value_nodes, report)
    }
}
