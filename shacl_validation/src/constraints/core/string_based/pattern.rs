use std::collections::HashSet;

use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
pub(crate) struct Pattern {
    pattern: String,
    flags: Option<String>,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Self {
        Pattern { pattern, flags }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Pattern {
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
