use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::context::ValidationContext;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::{
    constraints::{DefaultConstraintComponent, SparqlConstraintComponent},
    context::EvaluationContext,
    validate_error::ValidateError,
    validation_report::result::LazyValidationIterator,
    value_nodes::ValueNodes,
};

pub trait ComponentEvaluator<S: SRDFBasic, R: ValidatorRunner< S>> {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S, R>,
        evaluation_context: &EvaluationContext,
        value_nodes: ValueNodes<S>,
    ) -> Result<LazyValidationIterator< S>, ValidateError>;
}

impl<S: SRDF> ComponentEvaluator<S, DefaultValidatorRunner> for dyn DefaultConstraintComponent<S> {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S, DefaultValidatorRunner>,
        evaluation_context: &EvaluationContext,
        value_nodes: ValueNodes<S>,
    ) -> Result<LazyValidationIterator< S>, ValidateError> {
        Ok(self.evaluate_default(validation_context, evaluation_context, value_nodes))
    }
}

impl<S: QuerySRDF> ComponentEvaluator<S, QueryValidatorRunner>
    for dyn SparqlConstraintComponent<S>
{
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S, QueryValidatorRunner>,
        evaluation_context: &EvaluationContext,
        value_nodes: ValueNodes<S>,
    ) -> Result<LazyValidationIterator< S>, ValidateError> {
        Ok(self.evaluate_sparql(validation_context, evaluation_context, value_nodes))
    }
}
