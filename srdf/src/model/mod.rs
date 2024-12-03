use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

use conversions::*;
use iri_s::IriS;
use thiserror::Error;

pub mod conversions;
pub mod rdf;
pub mod reader;
pub mod sparql;

pub type TSubjectRef<'a, T> = <T as Triple>::SubjectRef<'a>;
pub type TPredicateRef<'a, T> = <T as Triple>::Iri<'a>;
pub type TObjectRef<'a, T> = <T as Triple>::TermRef<'a>;

pub trait Triple: Sized {
    /// Reference to the subject of a triple.
    type Subject<'a>: Subject + Copy
    where
        Self: 'a;

    /// Reference to the IRI of a triple.
    type Iri<'a>: Iri + Copy
    where
        Self: 'a;

    /// Reference to the term of a triple.
    type Term<'a>: Term + Copy
    where
        Self: 'a;

    /// The subject of this triple.
    fn subject<'a>(&'a self) -> Self::SubjectRef<'a>;

    /// The predicate of this triple.
    fn predicate<'a>(&'a self) -> Self::Iri<'a>;

    /// The object of this triple.
    fn object<'a>(&'a self) -> Self::TermRef<'a>;

    /// The components of this triple.
    fn spo<'a>(&'a self) -> (Self::SubjectRef<'a>, Self::Iri<'a>, Self::TermRef<'a>) {
        (self.subject(), self.predicate(), self.object())
    }
}

pub trait FromComponents: Triple {
    type Subject;
    type Predicate;
    type Object;

    /// Create a triple from its subject, predicate and object.
    fn from_spo(
        subject: impl Into<Self::Subject>,
        predicate: impl Into<Self::Predicate>,
        object: impl Into<Self::Object>,
    ) -> Self;
}

pub trait Subject: IntoTerm + Hash + Eq {
    /// Reference to the blank node of a subject.
    type BlankNodeRef<'x>: BlankNode + Copy
    where
        Self: 'x;

    /// Reference to the IRI of a subject.
    type IriRef<'x>: Iri + Copy
    where
        Self: 'x;

    #[cfg(feature = "rdf-star")]
    /// Reference to the triple of a subject.
    type TripleRef<'x>: Triple + Copy
    where
        Self: 'x;

    /// Whether this subject is a blank node.
    fn is_blank_node(&self) -> bool {
        self.into_blank_node().is_some()
    }

    /// Whether this subject is an IRI.
    fn is_iri(&self) -> bool {
        self.into_iri().is_some()
    }

    #[cfg(feature = "rdf-star")]
    /// Whether this subject is a triple.
    fn is_triple(&self) -> bool {
        self.into_triple().is_some()
    }

    /// Tranform this subject, returning it as a blank node.
    fn into_blank_node<'a>(&'a self) -> Option<Self::BlankNodeRef<'a>>;

    /// Tranform this subject, returning it as an IRI.
    fn into_iri<'a>(&'a self) -> Option<Self::IriRef<'a>>;

    #[cfg(feature = "rdf-star")]
    /// Tranform this subject, returning it as a triple.
    fn into_triple<'a>(&'a self) -> Option<Self::TripleRef<'a>>;
}

pub trait Iri: Hash + Eq {
    type IriRef<'x>: Iri + Copy
    where
        Self: 'x;

    /// Transform this IRI, returning it as an `IriS`.
    fn into_iri_s(&self) -> IriS;
}

pub trait Term: IntoSubject + Hash + Eq {
    /// Reference to the blank node of a term.
    type BlankNodeRef<'x>: BlankNode + Copy
    where
        Self: 'x;

    /// Reference to the IRI of a term.
    type IriRef<'x>: Iri + Copy
    where
        Self: 'x;

    /// Reference to the literal of a term.
    type LiteralRef<'x>: Literal + Copy
    where
        Self: 'x;

    #[cfg(feature = "rdf-star")]
    /// Reference to the triple of a term.
    type TripleRef<'x>: Triple + Copy
    where
        Self: 'x;

    /// Whether this term is a blank node.
    fn is_blank_node(&self) -> bool {
        self.into_blank_node().is_some()
    }

    /// Whether this term is an IRI.
    fn is_iri(&self) -> bool {
        self.into_iri().is_some()
    }

    /// Whether this term is a literal.
    fn is_literal(&self) -> bool {
        self.into_literal().is_some()
    }

    #[cfg(feature = "rdf-star")]
    /// Whether this term is a triple.
    fn is_triple(&self) -> bool {
        self.into_triple().is_some()
    }

    /// Transform this term, returning it as a blank node.
    fn into_blank_node<'a>(&'a self) -> Option<Self::BlankNodeRef<'a>>;

    /// Transform this term, returning it as an IRI.
    fn into_iri<'a>(&'a self) -> Option<Self::IriRef<'a>>;

    /// Transform this term, returning it as a literal.
    fn into_literal<'a>(&'a self) -> Option<Self::LiteralRef<'a>>;

    #[cfg(feature = "rdf-star")]
    /// Transform this term, returning it as a triple.
    fn into_triple<'a>(&'a self) -> Option<Self::TripleRef<'a>>;
}

pub trait BlankNode {
    fn id(&self) -> &str;
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
