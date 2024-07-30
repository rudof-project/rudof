use std::collections::HashSet;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
use crate::validation_report::report::ValidationReport;

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// https://www.w3.org/TR/shacl/#MinCountConstraintComponent
pub(crate) struct MinCount {
    min_count: isize,
}

impl MinCount {
    pub fn new(min_count: isize) -> Self {
        MinCount { min_count }
    }
}

impl<S> ConstraintComponent<S> for MinCount {
    fn evaluate(
        &self,
        _store: &S,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        if self.min_count == 0 {
            // If min_count is 0, then it always passes
            return Ok(());
        }

        if (value_nodes.len() as isize) < self.min_count {
            <MinCount as ConstraintComponent<S>>::make_validation_result(self, None, report);
        }

        Ok(())
    }
}
