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

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Xone {
    shapes: Vec<RDFNode>,
}

impl Xone {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        Xone { shapes }
    }
}

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for Xone {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Xone {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Xone {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
