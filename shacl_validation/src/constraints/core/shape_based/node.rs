use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::helper::shapes::get_shape_ref;
use crate::shape::ShapeValidator;
use crate::targets::Targets;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

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

impl<S: SRDFBasic> ConstraintComponent<S> for Node {
    fn evaluate<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        let shape = get_shape_ref(&self.shape, validation_context.schema()).expect("Missing Shape");

        let results = value_nodes.flat_map(move |(focus_node, value_node)| {
            let validate_context = ShapeValidator::new(shape, validation_context);
            let single_value_nodes = std::iter::once(value_node);
            let targets = Targets::new(single_value_nodes);
            let inner_results = validate_context.validate(Some(&targets));

            if inner_results.is_err() {
                Some(ValidationResult::new(
                    focus_node,
                    &evaluation_context,
                    Some(value_node),
                ))
            } else if inner_results.unwrap().peekable().peek().is_some() {
                Some(ValidationResult::new(
                    focus_node,
                    &evaluation_context,
                    Some(value_node),
                ))
            } else {
                None
            }
        });

        LazyValidationIterator::new(results)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for Node {
    fn evaluate_default<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Node {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
