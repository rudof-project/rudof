use prefixmap::IriRef;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::value_nodes::ValueNodes;

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Disjoint {
    iri_ref: IriRef,
}

impl Disjoint {
    pub fn new(iri_ref: IriRef) -> Self {
        Disjoint { iri_ref }
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for Disjoint {
    fn evaluate_default<'a>(
        &self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        unimplemented!()
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Disjoint {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        unimplemented!()
    }
}
