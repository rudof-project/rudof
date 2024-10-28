use iri_s::IriS;
use serde_derive::Serialize;
use std::fmt::Display;

/// SHACL paths follow the [SHACL property paths spec](https://www.w3.org/TR/shacl/#property-paths)
/// which are a subset of SPARQL property paths
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum SHACLPath {
    Predicate { pred: IriS },
    Alternative { paths: Vec<SHACLPath> },
    Sequence { paths: Vec<SHACLPath> },
    Inverse { path: Box<SHACLPath> },
    ZeroOrMore { path: Box<SHACLPath> },
    OneOrMore { path: Box<SHACLPath> },
    ZeroOrOne { path: Box<SHACLPath> },
}

impl SHACLPath {
    pub fn iri(pred: IriS) -> Self {
        SHACLPath::Predicate { pred }
    }
}

impl Display for SHACLPath {
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

impl From<SHACLPath> for &str {
    fn from(value: SHACLPath) -> Self {
        match value {
            SHACLPath::Predicate { .. } => todo!(),
            SHACLPath::Alternative { .. } => todo!(),
            SHACLPath::Sequence { .. } => todo!(),
            SHACLPath::Inverse { .. } => todo!(),
            SHACLPath::ZeroOrMore { .. } => todo!(),
            SHACLPath::OneOrMore { .. } => todo!(),
            SHACLPath::ZeroOrOne { .. } => todo!(),
        }
    }
}
