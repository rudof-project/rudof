use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::ValueNodes;

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Not {
    shape: RDFNode,
}

impl Not {
    pub fn new(shape: RDFNode) -> Self {
        Not { shape }
    }
}

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for Not {
    fn evaluate(
        &self,
        _validation_context: &ValidationContext<S>,
        _evaluation_context: EvaluationContext,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Not {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Not {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
