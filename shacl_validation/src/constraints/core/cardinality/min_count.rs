use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

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
    min_count: usize,
}

impl MinCount {
    pub fn new(min_count: isize) -> Self {
        MinCount {
            min_count: min_count as usize,
        }
    }
}

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for MinCount {
    fn evaluate(
        &self,
        _validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        if self.min_count == 0 {
            // If min_count is 0, then it always passes
            return Ok(ValidationResults::default());
        }

        let results = value_nodes
            .iter_focus_nodes()
            .filter_map(|(focus_node, value_nodes)| {
                if value_nodes.0.len() < self.min_count {
                    Some(ValidationResult::new(focus_node, &evaluation_context, None))
                } else {
                    None
                }
            });

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for MinCount {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for MinCount {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
