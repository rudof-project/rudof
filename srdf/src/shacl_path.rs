use iri_s::IriS;

/// SHACL paths follow the [SHACL property paths spec](https://www.w3.org/TR/shacl/#property-paths)
/// which are a subset of SPARQL property paths
///
#[derive(Debug)]
pub enum SHACLPath {
    Predicate { pred: IriS },
    Alternative { paths: Vec<SHACLPath> },
    Sequence { paths: Vec<SHACLPath> },
    Inverse { path: Box<SHACLPath> },
    ZeroOrMore { path: Box<SHACLPath> },
    OneOrMore { path: Box<SHACLPath> },
    ZeroOrOne { path: Box<SHACLPath> },
}
