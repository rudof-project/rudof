use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use iri_s::IriS;

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
    // type Triple: Triple;

    type Triple<'x>: Triple + Copy
    where
        Self: 'x;

    fn new(triple: Self::Triple<'_>, graph_name: GraphName) -> Self;
    fn triple(&self) -> Self::Triple<'_>;
    fn graph_name(&self) -> &GraphName;
}

pub trait Triple: Display {
    // type Subject: Subject + Eq + Hash + Clone + Display + Debug + TryFrom<Self::Term>;
    // type Iri: Iri + Eq + Hash + Clone + Display + Debug;
    // type Term: Term + Eq + Hash + Clone + Display + Debug + From<Self::Subject> + From<Self::Iri>;

    type Subject<'x>: Subject + Copy
    where
        Self: 'x;

    type Iri<'x>: Iri + Copy
    where
        Self: 'x;

    type Term<'x>: Term + Copy
    where
        Self: 'x;

    fn new(subj: Self::Subject<'_>, pred: Self::Iri<'_>, obj: Self::Term<'_>) -> Self;
    fn subj(&self) -> Self::Subject<'_>;
    fn pred(&self) -> Self::Iri<'_>;
    fn obj(&self) -> Self::Term<'_>;
}

pub trait Subject {
    // type BlankNode: BlankNode;
    // type Iri: Iri;
    // #[cfg(feature = "rdf-star")]
    // type Triple: Triple;

    type BlankNode<'x>: BlankNode + Copy
    where
        Self: 'x;

    type Iri<'x>: Iri + Copy
    where
        Self: 'x;

    #[cfg(feature = "rdf-star")]
    type Triple<'x>: Triple + Copy
    where
        Self: 'x;

    fn is_blank_node(&self) -> bool;
    fn is_iri(&self) -> bool;
    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool;

    fn as_blank_node(&self) -> Option<Self::BlankNode<'_>>;
    fn as_iri(&self) -> Option<Self::Iri<'_>>;
    #[cfg(feature = "rdf-star")]
    fn as_triple(&self) -> Option<Self::Triple<'_>>;
}

pub trait Iri {
    fn new(str: &str) -> Self;
    fn as_iri_s(&self) -> IriS;
}

pub trait Term {
    // type BlankNode: BlankNode + ToString;
    // type Iri: Iri + Clone + Eq + Display + Debug + Hash;
    // type Literal: Literal + Clone + Eq + Display + Debug + Hash;
    // #[cfg(feature = "rdf-star")]
    // type Triple: Triple;

    type BlankNode<'x>: BlankNode + Copy
    where
        Self: 'x;

    type Iri<'x>: Iri + Copy
    where
        Self: 'x;

    type Literal<'x>: Literal + Copy
    where
        Self: 'x;

    #[cfg(feature = "rdf-star")]
    type Triple<'x>: Triple + Copy
    where
        Self: 'x;

    fn is_blank_node(&self) -> bool;
    fn is_iri(&self) -> bool;
    fn is_literal(&self) -> bool;
    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool;

    fn as_blank_node(&self) -> Option<Self::BlankNode<'_>>;
    fn as_iri(&self) -> Option<Self::Iri<'_>>;
    fn as_literal(&self) -> Option<Self::Literal<'_>>;
    #[cfg(feature = "rdf-star")]
    fn as_triple(&self) -> Option<Self::Triple<'_>>;
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
