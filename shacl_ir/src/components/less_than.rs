use iri_s::IriS;
use std::fmt::Display;

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[derive(Debug, Clone)]
pub struct LessThan {
    iri: IriS,
}

impl LessThan {
    pub fn new(iri: IriS) -> Self {
        LessThan { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

impl Display for LessThan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LessThan: {}", self.iri())
    }
}
