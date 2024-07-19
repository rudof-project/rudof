use prefixmap::IriRef;
use shacl_ast::value::Value;

use crate::{constraints::Evaluate, validation_report::ValidationResult};

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
pub(crate) struct InConstraintComponent {
    values: Vec<Value>,
}

impl InConstraintComponent {
    pub fn new(values: Vec<Value>) -> Self {
        InConstraintComponent { values }
    }
}

impl Evaluate for InConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}

/// The RDF data model offers a huge amount of flexibility. Any node can in
/// principle have values for any property. However, in some cases it makes
/// sense to specify conditions on which properties can be applied to nodes.
/// The SHACL Core language includes a property called sh:closed that can be
/// used to specify the condition that each value node has values only for
/// those properties that have been explicitly enumerated via the property
/// shapes specified for the shape via sh:property.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
pub(crate) struct ClosedConstraintComponent {
    is_closed: bool,
    ignored_properties: Vec<IriRef>,
}

impl ClosedConstraintComponent {
    pub fn new(is_closed: bool, ignored_properties: Vec<IriRef>) -> Self {
        ClosedConstraintComponent {
            is_closed,
            ignored_properties,
        }
    }
}

impl Evaluate for ClosedConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
pub(crate) struct HasValueConstraintComponent {
    value: Value,
}

impl HasValueConstraintComponent {
    pub fn new(value: Value) -> Self {
        HasValueConstraintComponent { value }
    }
}

impl Evaluate for HasValueConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}
