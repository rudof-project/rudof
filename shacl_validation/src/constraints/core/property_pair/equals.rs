use std::collections::HashSet;

use prefixmap::IriRef;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Equals {
    iri_ref: IriRef,
}

impl Equals {
    pub fn new(iri_ref: IriRef) -> Self {
        Equals { iri_ref }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Equals {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
