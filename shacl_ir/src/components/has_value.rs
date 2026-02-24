use rudof_rdf::rdf_core::term::Object;
use std::fmt::Display;

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[derive(Debug, Clone)]
pub struct HasValue {
    value: Object,
}

impl HasValue {
    pub fn new(value: Object) -> Self {
        HasValue { value }
    }

    pub fn value(&self) -> &Object {
        &self.value
    }
}

impl Display for HasValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HasValue: {}", self.value())
    }
}
