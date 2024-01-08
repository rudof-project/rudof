use std::fmt::{Debug, Display};

use crate::literal::Literal;
use iri_s::IriS;
use serde_derive::{Deserialize, Serialize};

/*trait RDF {
    fn parse(format: RDFFormat) -> Self;
}*/

/// Posible RDF formats
pub enum RDFFormat {
    Turtle,
    NTriples,
    RDFXML,
}

pub enum Subject {
    Iri { iri: IriS },
    BlankNode(String),
}

/*pub struct Triple {
    pub subject: Subject,
    pub predicate: IriS,
    pub object: Object,
}*/

pub type RDFNode = Object;

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Object {
    Iri { iri: IriS },
    BlankNode(String),
    Literal(Literal),
}
impl Object {
    pub fn iri(iri: IriS) -> Object {
        Object::Iri { iri }
    }

    pub fn bnode(str: String) -> Object {
        Object::BlankNode(str)
    }

    pub fn literal(lit: Literal) -> Object {
        Object::Literal(lit)
    }
}

impl From<IriS> for Object {
    fn from(iri: IriS) -> Self {
        Object::Iri { iri }
    }
}

impl From<Literal> for Object {
    fn from(lit: Literal) -> Self {
        Object::Literal(lit)
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::Iri {
            iri: IriS::default(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri { iri } => write!(f, "{iri}"),
            Object::BlankNode(bnode) => write!(f, "_{bnode}"),
            Object::Literal(lit) => write!(f, "{lit}"),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri { iri } => write!(f, "Iri {{{iri:?}}}"),
            Object::BlankNode(bnode) => write!(f, "Bnode{{{bnode:?}}}"),
            Object::Literal(lit) => write!(f, "Literal{{{lit:?}}}"),
        }
    }
}
