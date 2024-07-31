use std::collections::HashSet;

use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
pub(crate) struct MaxLength {
    max_length: isize,
}

impl MaxLength {
    pub fn new(max_length: isize) -> Self {
        MaxLength { max_length }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for MaxLength {
    fn evaluate(
        &self,
        _store: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if S::term_is_bnode(node) {
                self.make_validation_result(Some(node), report);
            } else {
                return Err(ConstraintError::NotImplemented);
            }
        }
        Ok(())
    }
}
