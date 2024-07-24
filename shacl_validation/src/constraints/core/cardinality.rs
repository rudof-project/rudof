use std::collections::HashSet;

use srdf::{RDFNode, SRDFGraph};

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    validation_report::{report::ValidationReport, result::ValidationResult},
};

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// https://www.w3.org/TR/shacl/#MinCountConstraintComponent
pub(crate) struct MinCountConstraintComponent {
    min_count: isize,
}

impl MinCountConstraintComponent {
    pub fn new(min_count: isize) -> Self {
        MinCountConstraintComponent { min_count }
    }
}

impl Evaluate for MinCountConstraintComponent {
    fn evaluate(
        &self,
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        if self.min_count == 0 {
            // If min_count is 0, then it always passes
            return Ok(());
        }

        for node in value_nodes {}

        Ok(())
    }
}

/// sh:maxCount specifies the maximum number of value nodes that satisfy the
/// condition.
///
/// https://www.w3.org/TR/shacl/#MaxCountConstraintComponent
pub(crate) struct MaxCountConstraintComponent {
    max_count: isize,
}

impl MaxCountConstraintComponent {
    pub fn new(max_count: isize) -> Self {
        MaxCountConstraintComponent { max_count }
    }
}

impl Evaluate for MaxCountConstraintComponent {
    fn evaluate(
        &self,
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}
