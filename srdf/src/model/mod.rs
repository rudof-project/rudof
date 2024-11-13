pub mod rdf;
pub mod sparql;
pub mod async_rdf;
pub mod rdf_format;

pub trait Subject {
    type BlankNode: BlankNode;
    type Iri: Iri;
    type Triple: Triple;

    fn is_blank_node(&self) -> bool;
    fn is_iri(&self) -> bool;
    fn is_triple(&self) -> bool;

    fn as_blank_node(&self) -> Option<&Self::BlankNode>;
    fn as_iri(&self) -> Option<&Self::Iri>;
    fn as_triple(&self) -> Option<&Self::Triple>;
}

pub trait Iri {
    fn new(str: &str) -> Self;
}

pub trait Term {
    type BlankNode: BlankNode;
    type Iri: Iri;
    type Literal;
    type Triple: Triple;

    fn is_blank_node(&self) -> bool;
    fn is_iri(&self) -> bool;
    fn is_literal(&self) -> bool;
    fn is_triple(&self) -> bool;

    fn as_blank_node(&self) -> Option<&Self::BlankNode>;
    fn as_iri(&self) -> Option<&Self::Iri>;
    fn as_literal(&self) -> Option<&Self::Literal>;
    fn as_triple(&self) -> Option<&Self::Triple>;
}

pub trait Triple {
    type Subject: Subject + PartialEq;
    type Iri: Iri + PartialEq;
    type Term: Term + PartialEq;

    fn new(subj: Self::Subject, pred: Self::Iri, obj: Self::Term) -> Self;
    fn subj(&self) -> &Self::Subject;
    fn pred(&self) -> &Self::Iri;
    fn obj(&self) -> &Self::Term;
}

pub trait BlankNode {
    fn label(&self) -> &str;
}