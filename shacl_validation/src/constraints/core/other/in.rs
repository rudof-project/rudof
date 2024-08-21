use shacl_ast::value::Value;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;
use std::sync::Arc;

use crate::constraints::SparqlConstraintComponent;
use crate::constraints::{ConstraintComponent, DefaultConstraintComponent};
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
pub(crate) struct In<S: SRDFBasic> {
    values: Vec<S::Term>,
}

impl<S: SRDFBasic> In<S> {
    pub fn new(values: Vec<Value>) -> Self {
        In {
            values: values
                .iter()
                .map(|value| match value {
                    Value::Iri(iri_ref) => S::iri_s2term(&iri_ref.get_iri().unwrap()),
                    Value::Literal(lit) => S::object_as_term(&RDFNode::literal(lit.to_owned())),
                })
                .collect(),
        }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for In<S> {
    fn evaluate<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        let results = value_nodes.flat_map(move |(focus_node, value_node)| {
            if !self.values.contains(&value_node) {
                Some(ValidationResult::new(focus_node, &evaluation_context, None))
            } else {
                None
            }
        });

        LazyValidationIterator::new(results)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for In<S> {
    fn evaluate_default<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for In<S> {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
