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
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Or {
    shapes: Vec<RDFNode>,
}

impl Or {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        Or { shapes }
    }
}

impl<S> ConstraintComponent<S> for Or {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
