use std::collections::HashSet;

use prefixmap::IriRef;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
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

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for LessThan {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
