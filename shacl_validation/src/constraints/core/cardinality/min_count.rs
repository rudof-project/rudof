use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use std::sync::Arc;

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

impl< S: SRDFBasic, R: ValidatorRunner< S>> ConstraintComponent< S, R> for MinCount {
    fn evaluate(
        & self,
        validation_context: Arc<ValidationContext< S, R>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        if self.min_count == 0 {
            // If min_count is 0, then it always passes
            return LazyValidationIterator::default();
        }

        let results = value_nodes
            .iter_outer()
            .filter_map(move |(focus_node, value_nodes)| {
                if (value_nodes.count() as isize) < self.min_count {
                    Some(ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        None,
                    ))
                } else {
                    None
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl< S: SRDF> DefaultConstraintComponent< S> for MinCount {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for MinCount {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
