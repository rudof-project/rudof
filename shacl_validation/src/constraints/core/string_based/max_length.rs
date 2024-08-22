use indoc::formatdoc;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::ValueNodes;

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
pub(crate) struct MaxLength {
    max_length: isize,
}

impl MaxLength {
    pub fn new(max_length: isize) -> Self {
        MaxLength { max_length }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for MaxLength {
    fn evaluate_default<'a>(
        &self,
        _validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        let results = value_nodes
            .iter()
            .flat_map(move |(focus_node, value_node)| {
                if S::term_is_bnode(value_node) {
                    let result =
                        ValidationResult::new(focus_node, &evaluation_context, Some(value_node));
                    Some(result)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(LazyValidationIterator::new(results.into_iter()))
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for MaxLength {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        let results = value_nodes
            .iter()
            .filter_map(move |(focus_node, value_node)| {
                if S::term_is_bnode(value_node) {
                    Some(ValidationResult::new(
                        focus_node,
                        &evaluation_context,
                        Some(value_node),
                    ))
                } else {
                    let query = formatdoc! {
                        " ASK {{ FILTER (STRLEN(str({})) <= {}) }} ",
                        value_node, self.max_length
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
                }
            })
            .collect::<Vec<_>>();

        Ok(LazyValidationIterator::new(results.into_iter()))
    }
}
