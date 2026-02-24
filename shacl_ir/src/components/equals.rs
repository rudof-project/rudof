use iri_s::IriS;
use std::fmt::Display;

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[derive(Debug, Clone)]
pub struct Equals {
    iri: IriS,
}

impl Equals {
    pub fn new(iri: IriS) -> Self {
        Equals { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

impl Display for Equals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Equals: {}", self.iri())
    }
}
