use iri_s::IriS;
use std::fmt::Display;

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[derive(Debug, Clone)]
pub struct Disjoint {
    iri: IriS,
}

impl Disjoint {
    pub fn new(iri: IriS) -> Self {
        Disjoint { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

impl Display for Disjoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Disjoint: {}", self.iri())
    }
}
