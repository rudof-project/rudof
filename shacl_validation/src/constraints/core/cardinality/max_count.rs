use std::collections::HashSet;

use srdf::SRDFBasic;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:maxCount specifies the maximum number of value nodes that satisfy the
/// condition.
///
/// https://www.w3.org/TR/shacl/#MaxCountConstraintComponent
pub(crate) struct MaxCount {
    max_count: isize,
}

impl MaxCount {
    pub fn new(max_count: isize) -> Self {
        MaxCount { max_count }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for MaxCount {
    fn evaluate(
        &self,
        _store: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        if (value_nodes.len() as isize) > self.max_count {
            self.make_validation_result(None, report);
        }
        Ok(())
    }
}
