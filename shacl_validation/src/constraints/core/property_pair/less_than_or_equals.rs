use std::collections::HashSet;

use prefixmap::IriRef;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
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

impl<S> ConstraintComponent<S> for LessThanOrEquals {
    fn evaluate(
        &self,
        _: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
