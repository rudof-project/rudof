use rudof_iri::IriS;
use std::fmt::{Display, Formatter};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Datatype: {}", self.datatype())
    }
}
