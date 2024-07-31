use std::collections::HashSet;

use srdf::{RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
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

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Not {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
