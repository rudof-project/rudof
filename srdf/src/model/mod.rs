use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use iri_s::IriS;
use rdf::Predicate;
use rdf::Rdf;

pub mod focus_rdf;
pub mod mutable_rdf;
pub mod parse;
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
    type Subject: Subject + Eq + Hash + Clone + Display + Debug + TryFrom<Self::Term>;
    type Iri: Iri + Eq + Hash + Clone + Display + Debug;
    type Term: Term + Eq + Hash + Clone + Display + Debug + From<Self::Subject> + From<Self::Iri>;

    fn new(subj: Self::Subject, pred: Self::Iri, obj: Self::Term) -> Self;
    fn subj(&self) -> &Self::Subject;
    fn pred(&self) -> &Self::Iri;
    fn obj(&self) -> &Self::Term;
}

pub trait Subject {
    type BlankNode: BlankNode;
    type Iri: Iri;
    #[cfg(feature = "rdf-star")]
    type Triple: Triple;

    fn is_blank_node(&self) -> bool;
    fn is_iri(&self) -> bool;
    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool;

    fn as_blank_node(&self) -> Option<&Self::BlankNode>;
    fn as_iri(&self) -> Option<&Self::Iri>;
    #[cfg(feature = "rdf-star")]
    fn as_triple(&self) -> Option<&Self::Triple>;
}

pub trait Iri {
    fn new(str: &str) -> Self;
    fn as_iri_s(&self) -> IriS;
}

pub trait Term {
    type BlankNode: BlankNode + ToString;
    type Iri: Iri + Clone + Eq + Display + Debug + Hash;
    type Literal: Literal + Clone + Eq + Display + Debug + Hash;
    type Triple: Triple;

    fn is_blank_node(&self) -> bool;
    fn is_iri(&self) -> bool;
    fn is_literal(&self) -> bool;
    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool;

    fn as_blank_node(&self) -> Option<&Self::BlankNode>;
    fn as_iri(&self) -> Option<&Self::Iri>;
    fn as_literal(&self) -> Option<&Self::Literal>;
    #[cfg(feature = "rdf-star")]
    fn as_triple(&self) -> Option<&Self::Triple>;
}

pub trait BlankNode {
    fn label(&self) -> &str;
}

pub trait Literal {
    fn as_bool(&self) -> Option<bool>;
    fn as_string(&self) -> Option<String>;
    fn as_int(&self) -> Option<isize>;
    fn datatype(&self) -> &str;
}
