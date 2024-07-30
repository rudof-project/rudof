use std::collections::HashSet;

use prefixmap::IriRef;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
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

impl<S> ConstraintComponent<S> for Equals {
    fn evaluate(
        &self,
        _: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
