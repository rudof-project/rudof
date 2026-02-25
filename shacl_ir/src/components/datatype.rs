use iri_s::IriS;
use std::fmt::Display;

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug, Clone)]
pub struct Datatype {
    datatype: IriS,
}

impl Datatype {
    pub fn new(datatype: IriS) -> Self {
        Datatype { datatype }
    }

    pub fn datatype(&self) -> &IriS {
        &self.datatype
    }
}

impl Display for Datatype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Datatype: {}", self.datatype())
    }
}
