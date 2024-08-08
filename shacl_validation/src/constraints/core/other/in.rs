use std::collections::HashSet;

use shacl_ast::value::Value;
use shacl_ast::Schema;
use srdf::{QuerySRDF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::validation_report::report::ValidationReport;

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct In {
    values: Vec<Value>,
}

impl In {
    pub fn new(values: Vec<Value>) -> Self {
        In { values }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for In {
    fn evaluate_default(
        &self,
        _store: &S,
        _schema: &Schema,
        _: &DefaultValidatorRunner,
        _value_nodes: &HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for In {
    fn evaluate_sparql(
        &self,
        _store: &S,
        _schema: &Schema,
        _: &SparqlValidatorRunner,
        _value_nodes: &HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
