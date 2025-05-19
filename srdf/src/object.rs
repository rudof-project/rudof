use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};

use crate::literal::Literal;
use crate::numeric_literal::NumericLiteral;
use iri_s::IriS;
use serde::{Deserialize, Serialize};

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

    pub fn length(&self) -> usize {
        match self {
            Object::Iri(iri) => iri.as_str().len(),
            Object::BlankNode(bn) => bn.len(),
            Object::Literal(lit) => lit.lexical_form().len(),
        }
    }

    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        match self {
            Object::Iri(_) | Object::BlankNode(_) => None,
            Object::Literal(lit) => lit.numeric_value(),
        }
    }

    pub fn boolean(b: bool) -> Object {
        Object::Literal(Literal::boolean(b))
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

impl From<Object> for oxrdf::Term {
    fn from(value: Object) -> Self {
        match value {
            Object::Iri(iri_s) => oxrdf::NamedNode::new_unchecked(iri_s.as_str()).into(),
            Object::BlankNode(bnode) => oxrdf::BlankNode::new_unchecked(bnode).into(),
            Object::Literal(literal) => oxrdf::Term::Literal(literal.into()),
        }
    }
}

impl From<oxrdf::Term> for Object {
    fn from(value: oxrdf::Term) -> Self {
        match value {
            oxrdf::Term::NamedNode(named_node) => Object::iri(IriS::from_named_node(&named_node)),
            oxrdf::Term::BlankNode(blank_node) => Object::bnode(blank_node.into_string()),
            oxrdf::Term::Literal(literal) => Object::literal(literal.into()),
            #[cfg(feature = "rdf-star")]
            oxrdf::Term::Triple(_) => todo!(),
        }
    }
}

impl TryFrom<Object> for oxrdf::Subject {
    type Error = Infallible;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Iri(iri_s) => Ok(oxrdf::NamedNode::new_unchecked(iri_s.as_str()).into()),
            Object::BlankNode(bnode) => Ok(oxrdf::BlankNode::new_unchecked(bnode).into()),
            Object::Literal(_) => todo!(),
        }
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
