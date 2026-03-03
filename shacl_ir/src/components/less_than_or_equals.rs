use iri_s::IriS;
use std::fmt::Display;

/// LessThanOrEquals Constraint Component.
///
/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
#[derive(Debug, Clone)]
pub struct LessThanOrEquals {
    iri: IriS,
}

impl LessThanOrEquals {
    pub fn new(iri: IriS) -> Self {
        LessThanOrEquals { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

impl Display for LessThanOrEquals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LessThanOrEquals: {}", self.iri())
    }
}
