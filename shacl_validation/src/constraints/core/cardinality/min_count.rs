use std::collections::HashSet;

use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
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

impl<S: SRDFBasic> ConstraintComponent<S> for MinCount {
    fn evaluate(
        &self,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        if self.min_count == 0 {
            // If min_count is 0, then it always passes
            return Ok(());
        }
        if (value_nodes.len() as isize) < self.min_count {
            report.make_validation_result(None);
        }
        Ok(())
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for MinCount {
    fn evaluate_default(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for MinCount {
    fn evaluate_sparql(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}
