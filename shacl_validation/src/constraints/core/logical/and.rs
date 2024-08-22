use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::helper::shapes::get_shapes_ref;
use crate::shape::ShapeValidator;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::Targets;
use crate::ValueNodes;

/// sh:and specifies the condition that each value node conforms to all provided
/// shapes. This is comparable to conjunction and the logical "and" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
pub(crate) struct And {
    shapes: Vec<RDFNode>,
}

impl And {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        And { shapes }
    }
}

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for And {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        let results = value_nodes
            .iter()
            .flat_map(move |(focus_node, value_node)| {
                let all_valid = get_shapes_ref(&self.shapes, validation_context.schema())
                    .iter()
                    .flatten()
                    .all(|shape| {
                        let focus_nodes = Targets::new(std::iter::once(value_node.clone()));
                        let shape_validator =
                            ShapeValidator::new(shape, validation_context, Some(&focus_nodes));

                        let ans = match shape_validator.validate() {
                            Ok(results) => results.into_iter().peekable().peek().is_none(),
                            Err(_) => false,
                        };
                        ans
                    });

                if !all_valid {
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

        LazyValidationIterator::new(results.into_iter())
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for And {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for And {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
