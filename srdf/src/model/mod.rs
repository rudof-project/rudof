use std::fmt::Debug;
use std::str::FromStr;

use iri_s::IriS;
use thiserror::Error;

pub mod rdf;
pub mod reader;
pub mod sparql;

pub type TSubjectRef<'a, T> = <<T as Triple>::Subject as Subject>::SubjectRef<'a>;
pub type TPredicateRef<'a, T> = <<T as Triple>::Iri as Iri>::IriRef<'a>;
pub type TObjectRef<'a, T> = <<T as Triple>::Term as Term>::TermRef<'a>;

pub trait Triple: Sized {
    type TripleRef<'x>: Triple + Copy
    where
        Self: 'x;

    type Subject: Subject;
    type Iri: Iri;
    type Term: Term;

    /// Create a triple from its subject, predicate and object.
    fn from_spo(subject: Self::Subject, predicate: Self::Iri, object: Self::Term) -> Self;

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

    /// Whether this triple is equal to another triple.
    fn eq(&self, other: &Self) -> bool {
        self.subject().eq(&other.subject())
            && self.predicate().eq(&other.predicate())
            && self.object().eq(&other.object())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SubjectKind {
    BlankNode,
    Iri,
    #[cfg(feature = "rdf-star")]
    Triple,
}

pub trait Subject {
    type SubjectRef<'x>: Subject + Copy
    where
        Self: 'x;

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

    /// The kind of this subject.
    fn kind(&self) -> SubjectKind;

    /// Whether this subject is a blank node.
    fn is_blank_node(&self) -> bool {
        matches!(self.kind(), SubjectKind::BlankNode)
    }

    /// Whether this subject is an IRI.
    fn is_iri(&self) -> bool {
        matches!(self.kind(), SubjectKind::Iri)
    }

    #[cfg(feature = "rdf-star")]
    /// Whether this subject is a triple.
    fn is_triple(&self) -> bool {
        matches!(self.kind(), SubjectKind::Triple)
    }

    /// Tranform this subject, returning it as a blank node.
    fn into_blank_node(&self) -> Option<Self::BlankNode<'_>>;

    /// Tranform this subject, returning it as an IRI.
    fn into_iri(&self) -> Option<Self::Iri<'_>>;

    #[cfg(feature = "rdf-star")]
    /// Tranform this subject, returning it as a triple.
    fn into_triple(&self) -> Option<Self::Triple<'_>>;

    /// Whether this subject is equal to another subject.
    fn eq(&self, other: &Self) -> bool {
        if self.kind() != other.kind() {
            return false;
        }
        match self.kind() {
            SubjectKind::Iri => self.into_iri().unwrap().eq(&other.into_iri().unwrap()),
            SubjectKind::Triple => self
                .into_triple()
                .unwrap()
                .eq(&other.into_triple().unwrap()),
            SubjectKind::BlankNode => self
                .into_blank_node()
                .unwrap()
                .eq(&other.into_blank_node().unwrap()),
        }
    }
}

pub trait Iri {
    type IriRef<'x>: Iri + Copy
    where
        Self: 'x;

    /// Create an IRI from a string.
    fn from_str(s: &str) -> Self;

    /// Transform this IRI, returning it as an `IriS`.
    fn as_iri_s(&self) -> IriS;

    /// Whether this IRI is equal to another IRI.
    fn eq(&self, other: &Self) -> bool {
        self.as_iri_s() == other.as_iri_s()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TermKind {
    BlankNode,
    Iri,
    Literal,
    #[cfg(feature = "rdf-star")]
    Triple,
}

pub trait Term {
    type TermRef<'x>: Term + Copy
    where
        Self: 'x;

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

    /// The kind of this term.
    fn kind(&self) -> TermKind;

    /// Whether this term is a blank node.
    fn is_blank_node(&self) -> bool {
        matches!(self.kind(), TermKind::BlankNode)
    }

    /// Whether this term is an IRI.
    fn is_iri(&self) -> bool {
        matches!(self.kind(), TermKind::Iri)
    }

    /// Whether this term is a literal.
    fn is_literal(&self) -> bool {
        matches!(self.kind(), TermKind::Literal)
    }

    #[cfg(feature = "rdf-star")]
    /// Whether this term is a triple.
    fn is_triple(&self) -> bool {
        matches!(self.kind(), TermKind::Triple)
    }

    /// Transform this term, returning it as a blank node.
    fn into_blank_node(&self) -> Option<Self::BlankNode<'_>>;

    /// Transform this term, returning it as an IRI.
    fn into_iri(&self) -> Option<Self::Iri<'_>>;

    /// Transform this term, returning it as a literal.
    fn into_literal(&self) -> Option<Self::Literal<'_>>;

    #[cfg(feature = "rdf-star")]
    /// Transform this term, returning it as a triple.
    fn into_triple(&self) -> Option<Self::Triple<'_>>;

    /// Whether this term is equal to another term.
    fn eq(&self, other: &Self) -> bool {
        if self.kind() != other.kind() {
            return false;
        }
        match self.kind() {
            TermKind::Iri => self.into_iri().unwrap().eq(&other.into_iri().unwrap()),
            TermKind::Literal => self
                .into_literal()
                .unwrap()
                .eq(&other.into_literal().unwrap()),
            TermKind::BlankNode => self
                .into_blank_node()
                .unwrap()
                .eq(&other.into_blank_node().unwrap()),
            TermKind::Triple => self
                .into_triple()
                .unwrap()
                .eq(&other.into_triple().unwrap()),
        }
    }
}

pub trait BlankNode {
    fn id(&self) -> &str;

    /// Whether this blank node is equal to another blank node.
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

pub trait Literal {
    /// The datatype of this literal.
    fn datatype(&self) -> &str;

    /// Tranform this literal, returning it as a string.
    fn as_string(&self) -> Option<String>;

    /// Tranform this literal, returning it as a boolean.
    fn as_bool(&self) -> Option<bool> {
        match self.as_string() {
            Some(s) if s == "true" => Some(true),
            Some(s) if s == "false" => Some(false),
            _ => None,
        }
    }

    /// Tranform this literal, returning it as an integer.
    fn as_int(&self) -> Option<isize> {
        self.as_string().and_then(|s| s.parse().ok())
    }

    /// Tranform this literal, returning it as a float.
    fn as_float(&self) -> Option<f64> {
        self.as_string().and_then(|s| s.parse().ok())
    }

    /// Whether this literal is equal to another literal.
    fn eq(&self, other: &Self) -> bool {
        self.as_string() == other.as_string()
    }
}

#[derive(Error, Debug)]
#[error("Format {} not supported by RDF", ._0)]
pub struct FormatError(String);

/// Posible RDF formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum RdfFormat {
    #[default]
    Turtle,
    N3,
    RdfXml,
    NQuads,
    NTriples,
    TriG,
}

impl FromStr for RdfFormat {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<RdfFormat, FormatError> {
        match s {
            "ttl" => Ok(RdfFormat::Turtle),
            "nt" => Ok(RdfFormat::NTriples),
            "rdf" => Ok(RdfFormat::RdfXml),
            "trig" => Ok(RdfFormat::TriG),
            "n3" => Ok(RdfFormat::N3),
            "nq" => Ok(RdfFormat::NQuads),
            _ => Err(FormatError(s.to_string())),
        }
    }
}
