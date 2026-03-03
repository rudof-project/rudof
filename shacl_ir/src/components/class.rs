use rudof_rdf::rdf_core::term::Object;
use std::fmt::Display;

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug, Clone)]
pub struct Class {
    class_rule: Object,
}

impl Class {
    pub fn new(class_rule: Object) -> Self {
        Class { class_rule }
    }

    pub fn class_rule(&self) -> &Object {
        &self.class_rule
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class: {}", self.class_rule())
    }
}
