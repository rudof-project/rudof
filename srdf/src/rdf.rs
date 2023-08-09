use std::fmt::{Debug, Display};

use iri_s::IriS;
use serde_derive::{Deserialize, Serialize};
use crate::literal::Literal;

trait RDF {
    fn parse(format: RDFFormat) -> Self;
}

pub enum RDFFormat {
    Turtle,
    NTriples,
    RDFXML,
}

pub enum Subject {
    Iri { iri: IriS },
    BlankNode(String),
}

pub struct Triple {
    pub subject: Subject,
    pub predicate: IriS,
    pub object: Object,
}

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Object {
    Iri{ iri: IriS },
    BlankNode(String),
    Literal(Literal)
}

impl Default for Object {
    
    fn default() -> Self {
        Object::Iri { iri: IriS::default() }
    }
}

impl Display for Object {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri { iri } => write!(f, "{iri}"),
            Object::BlankNode(bnode) => write!(f, "_{bnode}"),
            Object::Literal(lit) => write!(f, "{lit}")
        }
    }
}

impl Debug for Object {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri { iri } => write!(f, "Iri {{{iri:?}}}"),
            Object::BlankNode(bnode) => write!(f, "Bnode{{{bnode:?}}}"),
            Object::Literal(lit) => write!(f, "Literal{{{lit:?}}}")
        }
    }
}