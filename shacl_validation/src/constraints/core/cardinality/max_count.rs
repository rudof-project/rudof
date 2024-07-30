use std::collections::HashSet;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
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

impl<S> ConstraintComponent<S> for MaxCount {
    fn evaluate(
        &self,
        _store: &S,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        if (value_nodes.len() as isize) > self.max_count {
            <MaxCount as ConstraintComponent<S>>::make_validation_result(self, None, report);
        }
        Ok(())
    }
}
