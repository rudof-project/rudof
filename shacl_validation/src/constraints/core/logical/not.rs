use std::collections::HashSet;

use srdf::RDFNode;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
use crate::validation_report::report::ValidationReport;

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Not {
    shape: RDFNode,
}

impl Not {
    pub fn new(shape: RDFNode) -> Self {
        Not { shape }
    }
}

impl<S> ConstraintComponent<S> for Not {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
