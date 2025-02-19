use srdf::model::rdf::Iri;
use srdf::model::Triple;
use std::fmt::Display;

/// SHACL paths follow the [SHACL property paths spec](https://www.w3.org/TR/shacl/#property-paths)
/// which are a subset of SPARQL property paths
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SHACLPath<T: Triple> {
    Predicate { pred: Iri<T> },
    Alternative { paths: Vec<SHACLPath<T>> },
    Sequence { paths: Vec<SHACLPath<T>> },
    Inverse { path: Box<SHACLPath<T>> },
    ZeroOrMore { path: Box<SHACLPath<T>> },
    OneOrMore { path: Box<SHACLPath<T>> },
    ZeroOrOne { path: Box<SHACLPath<T>> },
}

impl<T: Triple> SHACLPath<T> {
    pub fn iri(pred: Iri<T>) -> Self {
        SHACLPath::Predicate { pred }
    }
}

impl<T: Triple> Display for SHACLPath<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SHACLPath::Predicate { pred } => write!(f, "{pred}"),
            SHACLPath::Alternative { .. } => todo!(),
            SHACLPath::Sequence { .. } => todo!(),
            SHACLPath::Inverse { .. } => todo!(),
            SHACLPath::ZeroOrMore { .. } => todo!(),
            SHACLPath::OneOrMore { .. } => todo!(),
            SHACLPath::ZeroOrOne { .. } => todo!(),
        }
    }
}
