use std::collections::HashSet;

use shacl_ast::value::Value;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
use crate::validation_report::report::ValidationReport;

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct In {
    values: Vec<Value>,
}

impl In {
    pub fn new(values: Vec<Value>) -> Self {
        In { values }
    }
}

impl<S> ConstraintComponent<S> for In {
    fn evaluate(
        &self,
        _: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
