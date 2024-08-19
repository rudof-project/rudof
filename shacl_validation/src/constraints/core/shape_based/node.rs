use std::sync::Arc;

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
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
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

impl<S: SRDFBasic, R: ValidatorRunner<S>> ConstraintComponent<S, R> for Node {
    fn evaluate(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        let shape = get_shape_ref(&self.shape, validation_context.schema()).expect("Missing Shape");

        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                let single_value_nodes = std::iter::once(value_node.to_owned());
                let focus_nodes = Targets::new(single_value_nodes);
                let focus_nodes = Arc::new(focus_nodes);

                let validate_context = ShapeValidator::new(shape, Arc::clone(&validation_context));
                let inner_results = validate_context.validate(Arc::clone(&focus_nodes));

                if inner_results.is_err() {
                    Some(ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        Some(&value_node),
                    ))
                } else if inner_results.unwrap().peekable().peek().is_some() {
                    Some(ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        Some(&value_node),
                    ))
                } else {
                    None
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for Node {
    fn evaluate_default(
        &self,
        validation_context: Arc<ValidationContext<S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Node {
    fn evaluate_sparql(
        &self,
        validation_context: Arc<ValidationContext<S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
