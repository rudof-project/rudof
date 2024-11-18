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

pub type TSubjectRef<'a, T> = <<T as Triple>::Subject as Subject>::SubjectRef<'a>;
pub type TPredicateRef<'a, T> = <<T as Triple>::Iri as Iri>::IriRef<'a>;
pub type TObjectRef<'a, T> = <<T as Triple>::Term as Term>::TermRef<'a>;

pub type TLiteralRef<'a, T> = <<<T as Triple>::Term as Term>::Literal as Literal>::LiteralRef<'a>;
pub type TIriRef<'a, T> = <<<T as Triple>::Term as Term>::Iri as Iri>::IriRef<'a>;

pub enum GraphName {
    Default,
}

// pub trait Quad {
//     // type Triple: Triple;

//     type Triple<'x>: Triple + Copy
//     where
//         Self: 'x;

//     fn new(triple: TTriple<Self>, graph_name: GraphName) -> Self;
//     fn triple(&self) -> Self::Triple<'_>;
//     fn graph_name(&self) -> &GraphName;
// }

pub trait Triple: Display + Sized {
    type TripleRef<'x>: Triple + Copy
    where
        Self: 'x;

    type Subject: Subject + Eq + Hash + Clone + Display + Debug + TryFrom<Self::Term>;
    type Iri: Iri + Eq + Hash + Clone + Display + Debug;
    type Term: Term + Eq + Hash + Clone + Display + Debug + From<Self::Subject> + From<Self::Iri>;

    /// The subject of this triple.
    fn subject(&self) -> TSubjectRef<Self>;

    /// The predicate of this triple.
    fn predicate(&self) -> TPredicateRef<Self>;

    /// The object of this triple.
    fn object(&self) -> TObjectRef<Self>;

    /// Consume this triple, returning all its components.
    fn as_spo(self) -> (Self::Subject, Self::Iri, Self::Term);

    /// Consume this triple, returning its subject.
    fn as_subject(self) -> Self::Subject {
        let (s, _, _) = self.as_spo();
        s
    }

    /// Consume this triple, returning its predicate.
    fn as_predicate(self) -> Self::Iri {
        let (_, p, _) = self.as_spo();
        p
    }

    /// Consume this triple, returning its object.
    fn as_object(self) -> Self::Term {
        let (_, _, o) = self.as_spo();
        o
    }
}

pub trait Subject {
    type SubjectRef<'x>: Subject + Copy
    where
        Self: 'x;

    type BlankNode: BlankNode;
    type Iri: Iri;
    #[cfg(feature = "rdf-star")]
    type Triple: Triple;

    fn is_blank_node(&self) -> bool;
    fn is_iri(&self) -> bool;
    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool;

    fn blank_node(&self) -> Option<<Self::BlankNode as BlankNode>::BlankNodeRef<'_>>;
    fn iri(&self) -> Option<<Self::Iri as Iri>::IriRef<'_>>;
    #[cfg(feature = "rdf-star")]
    fn triple(&self) -> Option<<Self::Triple as Triple>::TripleRef<'_>>;
}

pub trait Iri {
    type IriRef<'x>: Iri + Copy
    where
        Self: 'x;

    fn new(str: &str) -> Self;
    fn into_iri_s(&self) -> IriS;
}

pub trait Term {
    type TermRef<'x>: Term + Copy
    where
        Self: 'x;

    type BlankNode: BlankNode + ToString;
    type Iri: Iri + Clone + Eq + Display + Debug + Hash;
    type Literal: Literal + Clone + Eq + Display + Debug + Hash;
    #[cfg(feature = "rdf-star")]
    type Triple: Triple;

    fn is_blank_node(&self) -> bool;
    fn is_iri(&self) -> bool;
    fn is_literal(&self) -> bool;
    #[cfg(feature = "rdf-star")]
    fn is_triple(&self) -> bool;

    fn blank_node(&self) -> Option<<Self::BlankNode as BlankNode>::BlankNodeRef<'_>>;
    fn iri(&self) -> Option<<Self::Iri as Iri>::IriRef<'_>>;
    fn literal(&self) -> Option<<Self::Literal as Literal>::LiteralRef<'_>>;
    #[cfg(feature = "rdf-star")]
    fn triple(&self) -> Option<<Self::Triple as Triple>::TripleRef<'_>>;
}

pub trait BlankNode {
    type BlankNodeRef<'x>: BlankNode + Copy
    where
        Self: 'x;

    fn label(&self) -> &str;
}

pub trait Literal {
    type LiteralRef<'x>: Literal + Copy
    where
        Self: 'x;

    fn datatype(&self) -> &str;

    fn as_bool(&self) -> Option<bool>;
    fn as_string(&self) -> Option<String>;
    fn as_int(&self) -> Option<isize>;
}
