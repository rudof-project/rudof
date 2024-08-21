use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::context::ValidationContext;
use crate::{
    constraints::{DefaultConstraintComponent, SparqlConstraintComponent},
    context::EvaluationContext,
    validate_error::ValidateError,
    validation_report::result::LazyValidationIterator,
    value_nodes::ValueNodes,
};

pub trait ComponentEvaluator<S: SRDFBasic> {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: &EvaluationContext,
        value_nodes: ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ValidateError>;
}

impl<S: SRDF> ComponentEvaluator<S, DefaultValidatorRunner> for dyn DefaultConstraintComponent<S> {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: &EvaluationContext,
        value_nodes: ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ValidateError> {
        Ok(self.evaluate_default(validation_context, evaluation_context, value_nodes))
    }
}

impl<S: QuerySRDF> ComponentEvaluator<S, QueryValidatorRunner>
    for dyn SparqlConstraintComponent<S>
{
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: &EvaluationContext,
        value_nodes: ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ValidateError> {
        Ok(self.evaluate_sparql(validation_context, evaluation_context, value_nodes))
    }
}
