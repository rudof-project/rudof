use indoc::formatdoc;
use srdf::literal::Literal;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
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

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
pub(crate) struct MinExclusive<S: SRDFBasic> {
    min_inclusive: S::Term,
}

impl<S: SRDFBasic> MinExclusive<S> {
    pub fn new(literal: Literal) -> Self {
        MinExclusive {
            min_inclusive: S::object_as_term(&RDFNode::literal(literal)),
        }
    }
}

impl< S: SRDF> DefaultConstraintComponent< S> for MinExclusive<S> {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        unimplemented!()
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for MinExclusive<S> {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        let results = value_nodes
            .iter_full()
            .filter_map(move |(focus_node, value_node)| {
                let query = formatdoc! {
                    " ASK {{ FILTER ({} < {}) }} ",
                    value_node, self.min_inclusive
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
            });

        LazyValidationIterator::new(results)
    }
}
