use std::collections::HashSet;

use iri_s::IriS;
use prefixmap::IriRef;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Datatype {
    datatype: IriS,
}

impl Datatype {
    pub fn new(iri_ref: IriRef) -> Self {
        Datatype {
            datatype: IriS::new_unchecked(&iri_ref.to_string()),
        }
    }
}

impl<S: SRDFBasic + SRDF> ConstraintComponent<S> for Datatype {
    fn evaluate(
        &self,
        _: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if let Some(literal) = S::term_as_literal(node) {
                if S::datatype(&literal) != S::iri_s2iri(&self.datatype) {
                    self.make_validation_result(Some(node), report);
                }
            } else {
                self.make_validation_result(Some(node), report);
            }
        }
        Ok(())
    }
}
