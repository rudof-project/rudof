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
use crate::helper::shapes::get_shape_ref;
use crate::shape::ShapeValidator;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::Targets;
use crate::ValueNodes;

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
pub(crate) struct Node {
    shape: RDFNode,
}

impl Node {
    pub fn new(shape: RDFNode) -> Self {
        Node { shape }
    }
}

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for Node {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        let shape = get_shape_ref(&self.shape, validation_context.schema()).expect("Missing Shape");

        let results = value_nodes
            .iter()
            .flat_map(move |(focus_node, value_node)| {
                let focus_nodes = Targets::new(std::iter::once(value_node.clone()));
                let shape_validator =
                    ShapeValidator::new(shape, validation_context, Some(&focus_nodes));

                let inner_results = shape_validator.validate();

                if inner_results.is_err()
                    || inner_results
                        .unwrap()
                        .into_iter()
                        .peekable()
                        .peek()
                        .is_some()
                {
                    Some(ValidationResult::new(
                        focus_node,
                        &evaluation_context,
                        Some(value_node),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(LazyValidationIterator::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Node {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Node {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
