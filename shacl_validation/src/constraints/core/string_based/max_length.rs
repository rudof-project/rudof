use indoc::formatdoc;
use srdf::QuerySRDF;
use srdf::SRDF;
use std::sync::Arc;

use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

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

impl<S: SRDF> DefaultConstraintComponent<S> for MaxLength {
    fn evaluate_default(
        &self,
        validation_context: Arc<ValidationContext<S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                if S::term_is_bnode(&value_node) {
                    let result = ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        Some(&value_node),
                    );
                    Some(result)
                } else {
                    None
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for MaxLength {
    fn evaluate_sparql(
        &self,
        validation_context: Arc<ValidationContext<S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        let results = value_nodes
            .iter_full()
            .filter_map(move |(focus_node, value_node)| {
                if S::term_is_bnode(&value_node) {
                    Some(ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        Some(&value_node),
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
                            &focus_node,
                            Arc::clone(&evaluation_context),
                            Some(&value_node),
                        ))
                    } else {
                        None
                    }
                }
            });

        LazyValidationIterator::new(results)
    }
}
