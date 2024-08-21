use indoc::formatdoc;
use srdf::literal::Literal;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
pub(crate) struct MaxInclusive<S: SRDFBasic> {
    max_inclusive: S::Term,
}

impl<S: SRDFBasic> MaxInclusive<S> {
    pub fn new(literal: Literal) -> Self {
        MaxInclusive {
            max_inclusive: S::object_as_term(&RDFNode::literal(literal)),
        }
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for MaxInclusive<S> {
    fn evaluate_default<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        unimplemented!()
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for MaxInclusive<S> {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        let results = value_nodes.filter_map(move |(focus_node, value_node)| {
            let query = formatdoc! {
                " ASK {{ FILTER ({} > {}) }} ",
                value_node, self.max_inclusive
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
        });

        LazyValidationIterator::new(results)
    }
}
