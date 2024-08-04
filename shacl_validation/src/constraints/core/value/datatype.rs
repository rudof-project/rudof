use std::collections::HashSet;

use iri_s::IriS;
use prefixmap::IriRef;
use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Datatype<S: SRDFBasic> {
    datatype: S::IRI,
}

impl<S: SRDFBasic> Datatype<S> {
    pub fn new(iri_ref: IriRef) -> Self {
        Datatype {
            datatype: S::iri_s2iri(&IriS::new_unchecked(&iri_ref.to_string())),
        }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for Datatype<S> {
    fn evaluate(
        &self,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if let Some(literal) = S::term_as_literal(node) {
                if S::datatype(&literal) != self.datatype {
                    report.make_validation_result(Some(node));
                }
            } else {
                report.make_validation_result(Some(node));
            }
        }
        Ok(())
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for Datatype<S> {
    fn evaluate_default(
        &self,
        _: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Datatype<S> {
    fn evaluate_sparql(
        &self,
        _: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}
