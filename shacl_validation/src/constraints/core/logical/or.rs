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

impl<S: SRDFBasic> ConstraintComponent<S> for Or {
    fn evaluate<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        let results = value_nodes.flat_map(move |(focus_node, value_node)| {
            let any_valid = get_shapes_ref(&self.shapes, validation_context.schema())
                .into_iter()
                .flatten()
                .any(|shape| {
                    match ShapeValidator::new(shape, validation_context)
                        .validate(Some(&Targets::new(std::iter::once(value_node))))
                    {
                        Ok(results) => results.peekable().peek().is_none(),
                        Err(_) => false,
                    }
                });

            if !any_valid {
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

impl<S: SRDF> DefaultConstraintComponent<S> for Or {
    fn evaluate_default<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Or {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
