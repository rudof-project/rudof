use std::collections::HashSet;

use srdf::RDFNode;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
use crate::validation_report::report::ValidationReport;

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Xone {
    shapes: Vec<RDFNode>,
}

impl Xone {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        Xone { shapes }
    }
}

impl<S> ConstraintComponent<S> for Xone {
    fn evaluate(
        &self,
        _: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
