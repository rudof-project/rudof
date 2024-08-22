use indoc::formatdoc;
use srdf::literal::Literal;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::ValueNodes;

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
pub(crate) struct MinInclusive<S: SRDFBasic> {
    min_inclusive: S::Term,
}

impl<S: SRDFBasic> MinInclusive<S> {
    pub fn new(literal: Literal) -> Self {
        MinInclusive {
            min_inclusive: S::object_as_term(&RDFNode::literal(literal)),
        }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for MinInclusive<S> {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for MinInclusive<S> {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        let results = value_nodes
            .iter()
            .filter_map(move |(focus_node, value_node)| {
                let query = formatdoc! {
                    " ASK {{ FILTER ({} <= {}) }} ",
                    value_node, self.min_inclusive
                };

                let ask = match validation_context.store().query_ask(&query) {
                    Ok(ask) => ask,
                    Err(_) => return None,
                };

                if !ask {
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
