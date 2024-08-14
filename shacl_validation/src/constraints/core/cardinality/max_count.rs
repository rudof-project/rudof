use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::constraints::ConstraintResult;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::shape::ValueNode;
use crate::validation_report::result::ValidationResult;

/// sh:maxCount specifies the maximum number of value nodes that satisfy the
/// condition.
///
/// - IRI: https://www.w3.org/TR/shacl/#MaxCountConstraintComponent
/// - DEF: If the number of value nodes is greater than $maxCount, there is a
///   validation result.
pub(crate) struct MaxCount {
    max_count: isize,
}

impl MaxCount {
    pub fn new(max_count: isize) -> Self {
        MaxCount { max_count }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for MaxCount {
    fn evaluate(
        &self,
        _: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        let mut results = Vec::new();
        for (focus_node, value_nodes) in value_nodes {
            if (value_nodes.len() as isize) > self.max_count {
                results.push(ValidationResult::new(focus_node, context, None));
            }
        }
        Ok(results)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for MaxCount {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for MaxCount {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}
