use std::sync::Arc;

use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::helper::shapes::get_shapes_ref;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::shape::ShapeValidator;
use crate::targets::Targets;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
pub(crate) struct Or {
    shapes: Vec<RDFNode>,
}

impl Or {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        Or { shapes }
    }
}

impl< S: SRDFBasic, R: ValidatorRunner< S>> ConstraintComponent< S, R> for Or {
    fn evaluate(
        & self,
        validation_context: Arc<ValidationContext< S, R>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        let shapes = get_shapes_ref(&self.shapes, validation_context.schema());

        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                let single_value_nodes = std::iter::once(value_node.to_owned());
                let focus_nodes = Targets::new(single_value_nodes);
                let focus_nodes = Arc::new(focus_nodes);

                let any_valid = shapes.iter().flatten().any(|shape| {
                    match ShapeValidator::new(shape, Arc::clone(&validation_context))
                        .validate(Arc::clone(&focus_nodes))
                    {
                        Ok(results) => results.peekable().peek().is_none(),
                        Err(_) => false,
                    }
                });

                if !any_valid {
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

impl< S: SRDF> DefaultConstraintComponent< S> for Or {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for Or {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
