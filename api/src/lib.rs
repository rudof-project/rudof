use std::fmt::Display;

pub mod mutable_rdf;
pub mod rdf;
pub mod rdf_format;
pub mod sparql;

pub enum GraphName {
    Default,
}

pub trait Quad {
    type Triple: Triple;

    fn new(triple: Self::Triple, graph_name: GraphName) -> Self;
    fn triple(&self) -> &Self::Triple;
    fn graph_name(&self) -> &GraphName;
}

pub trait Triple: Display {
    type Subject: Subject;
    type Iri: Iri;
    type Term: Term;

    fn new(subj: Self::Subject, pred: Self::Iri, obj: Self::Term) -> Self;
    fn subj(&self) -> &Self::Subject;
    fn pred(&self) -> &Self::Iri;
    fn obj(&self) -> &Self::Term;
}

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

pub trait BlankNode {
    fn label(&self) -> &str;
}
