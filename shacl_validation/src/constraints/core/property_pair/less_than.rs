use std::collections::HashSet;

use prefixmap::IriRef;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
use crate::validation_report::report::ValidationReport;

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct LessThan {
    iri_ref: IriRef,
}

impl LessThan {
    pub fn new(iri_ref: IriRef) -> Self {
        LessThan { iri_ref }
    }
}

impl<S> ConstraintComponent<S> for LessThan {
    fn evaluate(
        &self,
        _: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
