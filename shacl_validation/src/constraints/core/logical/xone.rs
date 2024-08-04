use std::collections::HashSet;

use srdf::{QuerySRDF, RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Xone {
    shapes: Vec<RDFNode>,
}

impl Xone {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        Xone { shapes }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for Xone {
    fn evaluate(
        &self,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for Xone {
    fn evaluate_default(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Xone {
    fn evaluate_sparql(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}
