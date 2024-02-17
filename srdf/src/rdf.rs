use std::fmt::{Debug, Display};

use crate::literal::Literal;
use iri_s::IriS;
use serde_derive::{Deserialize, Serialize};


/// Concrete representation of RDF subjects, which can be IRIs or Blank nodes
pub enum Subject {
    Iri { iri: IriS },
    BlankNode(String),
}

/// Concrete representation of RDF nodes, which are equivalent to objects
pub type RDFNode = Object;

/// Concrete representation of RDF objects which can be IRIs, Blank nodes or literals
/// 
/// Note: We plan to support triple terms as in RDF-star in the future
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Object {
    Iri(IriS),
    BlankNode(String),
    Literal(Literal),
}


impl Object {
    pub fn iri(iri: IriS) -> Object {
        Object::Iri(iri)
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
        Object::Iri(iri)
    }
}

impl From<Literal> for Object {
    fn from(lit: Literal) -> Self {
        Object::Literal(lit)
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::Iri(IriS::default())
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri(iri) => write!(f, "{iri}"),
            Object::BlankNode(bnode) => write!(f, "_{bnode}"),
            Object::Literal(lit) => write!(f, "{lit}"),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri(iri) => write!(f, "Iri {{{iri:?}}}"),
            Object::BlankNode(bnode) => write!(f, "Bnode{{{bnode:?}}}"),
            Object::Literal(lit) => write!(f, "Literal{{{lit:?}}}"),
        }
    }
}
