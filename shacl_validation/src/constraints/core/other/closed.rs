use std::collections::HashSet;

use prefixmap::IriRef;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
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

impl<S> ConstraintComponent<S> for Closed {
    fn evaluate(
        &self,
        _: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
