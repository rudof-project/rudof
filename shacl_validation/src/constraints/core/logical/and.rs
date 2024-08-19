use std::sync::Arc;

use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::targets::Targets;
use crate::helper::shapes::get_shapes_ref;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::shape::ShapeValidator;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

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

impl< S: SRDFBasic, R: ValidatorRunner< S>> ConstraintComponent< S, R> for And {
    fn evaluate(
        & self,
        validation_context: Arc<ValidationContext< S, R>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        let shapes = get_shapes_ref(&self.shapes, Arc::clone(&validation_context).schema());

        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                let single_value_nodes = std::iter::once(value_node.to_owned());
                let focus_nodes = Targets::new(single_value_nodes);
                let focus_nodes = Arc::new(focus_nodes);

                let all_valid = shapes.iter().flatten().all(|shape| {
                    match ShapeValidator::new(shape, Arc::clone(&validation_context))
                        .validate(Arc::clone(&focus_nodes))
                    {
                        Ok(results) => results.peekable().peek().is_none(),
                        Err(_) => false,
                    }
                });

                if !all_valid {
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

impl< S: SRDF> DefaultConstraintComponent< S> for And {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for And {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
