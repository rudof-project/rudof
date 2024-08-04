use std::collections::HashSet;

use shacl_ast::value::Value;
use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct HasValue {
    value: Value,
}

impl HasValue {
    pub fn new(value: Value) -> Self {
        HasValue { value }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for HasValue {
    fn evaluate(
        &self,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for HasValue {
    fn evaluate_default(
        &self,
        _store: &S,
        _value_nodes: HashSet<<S>::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for HasValue {
    fn evaluate_sparql(
        &self,
        _store: &S,
        _value_nodes: HashSet<<S>::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
