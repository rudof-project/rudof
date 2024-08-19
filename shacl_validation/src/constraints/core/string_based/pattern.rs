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

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
pub(crate) struct Pattern {
    pattern: String,
    flags: Option<String>,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Self {
        Pattern { pattern, flags }
    }
}

impl< S: SRDF> DefaultConstraintComponent< S> for Pattern {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
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

impl< S: QuerySRDF> SparqlConstraintComponent< S> for Pattern {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
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
                    let query = match &self.flags {
                        Some(flags) => formatdoc! {
                            "ASK {{ FILTER (regex(str({}), {}, {})) }}",
                            value_node, self.pattern, flags
                        },
                        None => formatdoc! {
                            "ASK {{ FILTER (regex(str({}), {})) }}",
                            value_node, self.pattern
                        },
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
