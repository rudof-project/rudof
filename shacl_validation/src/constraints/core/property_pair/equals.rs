use std::collections::HashSet;

use prefixmap::IriRef;
use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Equals {
    iri_ref: IriRef,
}

impl Equals {
    pub fn new(iri_ref: IriRef) -> Self {
        Equals { iri_ref }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for Equals {
    fn evaluate(
        &self,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for Equals {
    fn evaluate_default(
        &self,
        _: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Equals {
    fn evaluate_sparql(
        &self,
        _: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
