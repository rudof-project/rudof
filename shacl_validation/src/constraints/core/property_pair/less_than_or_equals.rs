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

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for LessThanOrEquals {
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

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for LessThanOrEquals {
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
