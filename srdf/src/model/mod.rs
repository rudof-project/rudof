use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

use iri_s::IriS;
use thiserror::Error;

pub mod parse;
pub mod rdf;
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

pub trait Triple: Sized + Hash + Eq + Clone + Debug + Display {
    type Subject: Subject + TryFrom<Self::Term>;
    type Iri: Iri;
    type Term: Term + From<Self::Subject> + From<Self::Iri>;

    /// Create a triple from its subject, predicate and object.
    fn from_spo(
        subj: impl Into<Self::Subject>,
        pred: impl Into<Self::Iri>,
        obj: impl Into<Self::Term>,
    ) -> Self;

    /// The subject of this triple.
    fn subject(&self) -> &Self::Subject;

    /// The predicate of this triple.
    fn predicate(&self) -> &Self::Iri;

    /// The object of this triple.
    fn object(&self) -> &Self::Term;

    /// The components of this triple.
    fn spo(&self) -> (&Self::Subject, &Self::Iri, &Self::Term) {
        (self.subject(), self.predicate(), self.object())
    }

    /// Consumes this triple and returns its components.
    fn into_spo(self) -> (Self::Subject, Self::Iri, Self::Term);

    /// Consumes this triple and returns its subject.
    fn into_subject(self) -> Self::Subject {
        let (s, _, _) = self.into_spo();
        s
    }

    /// Consumes this triple and returns its predicate.
    fn into_predicate(self) -> Self::Iri {
        let (_, p, _) = self.into_spo();
        p
    }

    /// Consumes this triple and returns its object.
    fn into_object(self) -> Self::Term {
        let (_, _, o) = self.into_spo();
        o
    }
}

pub trait Subject: Hash + Eq + Clone + Debug + Display {
    type BlankNode: BlankNode;
    type Iri: Iri;

    #[cfg(feature = "rdf-star")]
    type Triple: Triple;

    /// Whether this subject is a blank node.
    fn is_blank_node(&self) -> bool {
        self.blank_node().is_some()
    }

    /// Whether this subject is an IRI.
    fn is_iri(&self) -> bool {
        self.iri().is_some()
    }

    #[cfg(feature = "rdf-star")]
    /// Whether this subject is a triple.
    fn is_triple(&self) -> bool {
        self.triple().is_some()
    }

    /// Returns the blank node if this subject is a blank node.
    fn blank_node(&self) -> Option<&Self::BlankNode>;

    /// Returns the IRI if this subject is an IRI.
    fn iri(&self) -> Option<&Self::Iri>;

    #[cfg(feature = "rdf-star")]
    /// Returns the triple if this subject is a triple.
    fn triple(&self) -> Option<&Self::Triple>;
}

pub trait Iri: Hash + Eq + Clone + Debug + Display {
    /// Creates a new IRI from a string.
    fn from_str(str: &str) -> Self;

    /// Converts the IRI to an IriS.
    fn into_iri_s(&self) -> IriS;
}

pub trait Term: Hash + Eq + Clone + Debug + Display {
    type BlankNode: BlankNode;
    type Iri: Iri;
    type Literal: Literal;

    #[cfg(feature = "rdf-star")]
    type Triple: Triple;

    /// Whether the term is a blank node.
    fn is_blank_node(&self) -> bool {
        self.blank_node().is_some()
    }

    /// Whether the term is an IRI.
    fn is_iri(&self) -> bool {
        self.iri().is_some()
    }

    /// Whether the term is a literal.
    fn is_literal(&self) -> bool {
        self.literal().is_some()
    }

    #[cfg(feature = "rdf-star")]
    /// Whether the term is a triple.
    fn is_triple(&self) -> bool {
        self.triple().is_some()
    }

    /// Returns the blank node if the term is a blank node.
    fn blank_node(&self) -> Option<&Self::BlankNode>;

    /// Returns the IRI if the term is an IRI.
    fn iri(&self) -> Option<&Self::Iri>;

    /// Returns the literal if the term is a literal.
    fn literal(&self) -> Option<&Self::Literal>;

    #[cfg(feature = "rdf-star")]
    /// Returns the triple if the term is a triple.
    fn triple(&self) -> Option<&Self::Triple>;
}

pub trait BlankNode {
    /// Returns the label of the blank node.
    fn label(&self) -> &str;
}

pub trait Literal: Hash + Eq + Clone + Debug + Display {
    /// Returns the datatype of the literal.
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
