use prefixmap::IriRef;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::shape::ValueNode;
use crate::validation_report::report::ValidationReport;

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

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Disjoint {
    fn evaluate_default(
        &self,
        _executor: &DefaultExecutor<S>,
        _context: &Context,
        _value_nodes: &ValueNode<S>,
        _report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Disjoint {
    fn evaluate_sparql(
        &self,
        _executor: &QueryExecutor<S>,
        _context: &Context,
        _value_nodes: &ValueNode<S>,
        _report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
