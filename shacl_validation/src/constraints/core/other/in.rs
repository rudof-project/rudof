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
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
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

impl<S: SRDFBasic, R: ValidatorRunner<S>> ConstraintComponent<S, R> for In<S> {
    fn evaluate(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                if !self.values.contains(&value_node) {
                    Some(ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        None,
                    ))
                } else {
                    None
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for In<S> {
    fn evaluate_default(
        &self,
        validation_context: Arc<ValidationContext<S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for In<S> {
    fn evaluate_sparql(
        &self,
        validation_context: Arc<ValidationContext<S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
