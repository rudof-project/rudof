use std::collections::HashSet;

use prefixmap::IriRef;
use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// The RDF data model offers a huge amount of flexibility. Any node can in
/// principle have values for any property. However, in some cases it makes
/// sense to specify conditions on which properties can be applied to nodes.
/// The SHACL Core language includes a property called sh:closed that can be
/// used to specify the condition that each value node has values only for
/// those properties that have been explicitly enumerated via the property
/// shapes specified for the shape via sh:property.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Closed {
    is_closed: bool,
    ignored_properties: Vec<IriRef>,
}

impl Closed {
    pub fn new(is_closed: bool, ignored_properties: Vec<IriRef>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for Closed {
    fn evaluate(
        &self,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for Closed {
    fn evaluate_default(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Closed {
    fn evaluate_sparql(
        &self,
        _: &S,
        value_nodes: HashSet<<S>::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}
