use std::collections::HashSet;

use srdf::{RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Class {
    class_rule: Option<RDFNode>,
}

impl Class {
    pub fn new(class_rule: RDFNode) -> Self {
        let class_rule = match class_rule {
            RDFNode::Iri(i) => Some(RDFNode::Iri(i)),
            RDFNode::BlankNode(_) => None,
            RDFNode::Literal(_) => None,
        };
        Class { class_rule }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Class {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
