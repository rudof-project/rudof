use std::collections::HashSet;

use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
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
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        if (value_nodes.len() as isize) > self.max_count {
            report.make_validation_result(None);
        }
        Ok(())
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for MaxCount {
    fn evaluate_default(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for MaxCount {
    fn evaluate_sparql(
        &self,
        _: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}
