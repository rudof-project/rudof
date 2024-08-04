use std::collections::HashSet;

use srdf::{QuerySRDF, RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct And {
    shapes: Vec<RDFNode>,
}

impl And {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        And { shapes }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for And {
    fn evaluate(
        &self,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for And {
    fn evaluate_default(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for And {
    fn evaluate_sparql(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}
