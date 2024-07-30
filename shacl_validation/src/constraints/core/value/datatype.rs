use std::collections::HashSet;

use prefixmap::IriRef;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
use crate::validation_report::report::ValidationReport;

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Datatype {
    iri_ref: IriRef,
}

impl Datatype {
    pub fn new(iri_ref: IriRef) -> Self {
        Datatype { iri_ref }
    }
}

impl<S> ConstraintComponent<S> for Datatype {
    fn evaluate(
        &self,
        _: &S,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if let Term::Literal(literal) = node {
                if literal.datatype() != self.iri_ref {
                    <Datatype as ConstraintComponent<S>>::make_validation_result(
                        self,
                        Some(node),
                        report,
                    );
                }
            } else {
                <Datatype as ConstraintComponent<S>>::make_validation_result(
                    self,
                    Some(node),
                    report,
                );
            }
        }
        Ok(())
    }
}
