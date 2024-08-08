use std::collections::HashSet;

use prefixmap::IriRef;
use shacl_ast::Schema;
use srdf::{QuerySRDF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::validation_report::report::ValidationReport;

/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct LessThanOrEquals {
    iri_ref: IriRef,
}

impl LessThanOrEquals {
    pub fn new(iri_ref: IriRef) -> Self {
        LessThanOrEquals { iri_ref }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for LessThanOrEquals {
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

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for LessThanOrEquals {
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
