use std::collections::HashSet;

use srdf::{RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Node {
    shape: RDFNode,
}

impl Node {
    pub fn new(shape: RDFNode) -> Self {
        Node { shape }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Node {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
