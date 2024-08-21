use prefixmap::IriRef;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::value_nodes::ValueNodes;

/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct LessThanOrEquals {
    iri_ref: IriRef,
}

impl LessThanOrEquals {
    pub fn new(iri_ref: IriRef) -> Self {
        LessThanOrEquals { iri_ref }
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for LessThanOrEquals {
    fn evaluate_default<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        unimplemented!()
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for LessThanOrEquals {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        unimplemented!()
    }
}
