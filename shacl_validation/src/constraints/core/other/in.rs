use std::collections::HashSet;

use shacl_ast::value::Value;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
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

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for In {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
