use std::collections::HashSet;

use srdf::literal::Literal;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// https://www.w3.org/TR/shacl/#MaxExclusiveConstraintComponent
pub(crate) struct MaxExclusive {
    literal: Literal,
}

impl MaxExclusive {
    pub fn new(literal: Literal) -> Self {
        MaxExclusive { literal }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for MaxExclusive {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
