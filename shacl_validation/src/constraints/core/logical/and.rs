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
use crate::targets::Targets;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

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

impl<S: SRDFBasic> ConstraintComponent<S> for And {
    fn evaluate<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        let results = value_nodes.flat_map(move |(focus_node, value_node)| {
            let all_valid = get_shapes_ref(&self.shapes, validation_context.schema())
                .into_iter()
                .flatten()
                .all(|shape| {
                    match ShapeValidator::new(shape, validation_context)
                        .validate(Some(&Targets::new(std::iter::once(value_node))))
                    {
                        Ok(results) => results.peekable().peek().is_none(),
                        Err(_) => false,
                    }
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
        });

        LazyValidationIterator::new(results)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for And {
    fn evaluate_default<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for And {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
