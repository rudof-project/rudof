use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::ValueNodes;

use itertools::Itertools;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

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

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for MinCount {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        if self.min_count == 0 {
            // If min_count is 0, then it always passes
            return LazyValidationIterator::default();
        }

        let results = value_nodes
            .iter()
            .chunk_by(|(focus_node, _)| focus_node.clone())
            .into_iter()
            .filter_map(move |(focus_node, value_nodes)| {
                if (value_nodes.count() as isize) < self.min_count {
                    Some(ValidationResult::new(focus_node, &evaluation_context, None))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        LazyValidationIterator::new(results.into_iter())
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for MinCount {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for MinCount {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
