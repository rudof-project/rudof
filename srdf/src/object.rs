use std::fmt::{Debug, Display};

use crate::RDFError;
use crate::literal::SLiteral;
use crate::numeric_literal::NumericLiteral;
use crate::triple::Triple;
use iri_s::IriS;
use prefixmap::IriRef;
use serde::{Deserialize, Serialize};

/// Concrete representation of RDF objects which can be IRIs, Blank nodes, literals or triples
///
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Object {
    Iri(IriS),
    BlankNode(String),
    Literal(SLiteral),
    Triple {
        subject: Box<IriOrBlankNode>,
        predicate: Box<IriS>,
        object: Box<Object>,
    },
}

impl Object {
    pub fn iri(iri: IriS) -> Object {
        Object::Iri(iri)
    }

    pub fn bnode(str: String) -> Object {
        Object::BlankNode(str)
    }

    pub fn literal(lit: SLiteral) -> Object {
        Object::Literal(lit)
    }

    pub fn str(str: &str) -> Object {
        Object::Literal(SLiteral::str(str))
    }

    pub fn length(&self) -> usize {
        match self {
            Object::Iri(iri) => iri.as_str().len(),
            Object::BlankNode(bn) => bn.len(),
            Object::Literal(lit) => lit.lexical_form().len(),
            Object::Triple {
                subject,
                predicate,
                object,
            } => {
                subject.as_ref().length()
                    + predicate.as_ref().as_str().len()
                    + object.as_ref().length()
            }
        }
    }

    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        match self {
            Object::Literal(lit) => lit.numeric_value(),
            _ => None,
        }
    }

    pub fn boolean(b: bool) -> Object {
        Object::Literal(SLiteral::boolean(b))
    }

    pub fn datatype(&self) -> Option<IriRef> {
        match self {
            Object::Literal(lit) => Some(lit.datatype()),
            _ => None,
        }
    }
}

impl From<IriS> for Object {
    fn from(iri: IriS) -> Self {
        Object::Iri(iri)
    }
}

impl From<SLiteral> for Object {
    fn from(lit: SLiteral) -> Self {
        Object::Literal(lit)
    }
}

impl From<Object> for oxrdf::Term {
    fn from(value: Object) -> Self {
        match value {
            Object::Iri(iri_s) => oxrdf::NamedNode::new_unchecked(iri_s.as_str()).into(),
            Object::BlankNode(bnode) => oxrdf::BlankNode::new_unchecked(bnode).into(),
            Object::Literal(literal) => oxrdf::Term::Literal(literal.into()),
            Object::Triple { .. } => todo!(),
        }
    }
}

/*impl From<oxrdf::Term> for Object {
    fn from(value: oxrdf::Term) -> Self {
        println!("Converting oxrdf::Term to Object: {value:?}");
        match value {
            oxrdf::Term::NamedNode(named_node) => Object::iri(IriS::from_named_node(&named_node)),
            oxrdf::Term::BlankNode(blank_node) => Object::bnode(blank_node.into_string()),
            oxrdf::Term::Literal(literal) => Object::literal(literal.into()),
            #[cfg(feature = "rdf-star")]
            oxrdf::Term::Triple(_) => todo!(),
        }
    }
}*/

impl TryFrom<oxrdf::Term> for Object {
    // TODO: Change this to a more appropriate error type
    type Error = RDFError;

    fn try_from(value: oxrdf::Term) -> Result<Self, Self::Error> {
        match value {
            oxrdf::Term::NamedNode(named_node) => {
                Ok(Object::iri(IriS::from_named_node(&named_node)))
            }
            oxrdf::Term::BlankNode(blank_node) => Ok(Object::bnode(blank_node.into_string())),
            oxrdf::Term::Literal(literal) => {
                let lit: SLiteral = literal.try_into()?;
                Ok(Object::literal(lit))
            }
            oxrdf::Term::Triple(triple) => {
                let (s, p, o) = triple.into_components();
                let object = Object::try_from(o)?;
                let subject = IriOrBlankNode::from(s);
                let predicate = IriS::from_named_node(&p);
                Ok(Object::Triple {
                    subject: Box::new(subject),
                    predicate: Box::new(predicate),
                    object: Box::new(object),
                })
            }
        }
    }
}

impl TryFrom<Object> for oxrdf::NamedOrBlankNode {
    // TODO: Change this to a more appropriate error type
    type Error = RDFError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        println!("Trying from Object: {value}");
        match value {
            Object::Iri(iri_s) => Ok(oxrdf::NamedNode::new_unchecked(iri_s.as_str()).into()),
            Object::BlankNode(bnode) => Ok(oxrdf::BlankNode::new_unchecked(bnode).into()),
            Object::Literal(_) => todo!(),
            Object::Triple { .. } => todo!(),
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
            Object::Triple { .. } => todo!(),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri(iri) => write!(f, "Iri {{{iri:?}}}"),
            Object::BlankNode(bnode) => write!(f, "Bnode{{{bnode:?}}}"),
            Object::Literal(lit) => write!(f, "Literal{{{lit:?}}}"),
            Object::Triple {
                subject,
                predicate,
                object,
            } => write!(f, "Triple {{{subject:?}, {predicate:?}, {object:?}}}"),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Iri(a), Object::Iri(b)) => a.partial_cmp(b),
            (Object::BlankNode(a), Object::BlankNode(b)) => a.partial_cmp(b),
            (Object::Literal(a), Object::Literal(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum IriOrBlankNode {
    BlankNode(String),
    Iri(IriS),
}

impl IriOrBlankNode {
    pub fn length(&self) -> usize {
        match self {
            IriOrBlankNode::BlankNode(label) => label.len(),
            IriOrBlankNode::Iri(iri) => iri.as_str().len(),
        }
    }

    pub fn iri(iri: &IriS) -> IriOrBlankNode {
        IriOrBlankNode::Iri(iri.clone())
    }
}

impl Display for IriOrBlankNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriOrBlankNode::BlankNode(b) => write!(f, "{b}"),
            IriOrBlankNode::Iri(iri_s) => write!(f, "{iri_s}"),
        }
    }
}

impl From<IriOrBlankNode> for oxrdf::NamedOrBlankNode {
    fn from(value: IriOrBlankNode) -> Self {
        match value {
            IriOrBlankNode::Iri(iri) => oxrdf::NamedNode::new_unchecked(iri.as_str()).into(),
            IriOrBlankNode::BlankNode(bnode) => oxrdf::BlankNode::new_unchecked(bnode).into(),
        }
    }
}

impl TryFrom<Object> for IriOrBlankNode {
    type Error = RDFError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Iri(iri) => Ok(IriOrBlankNode::Iri(iri)),
            Object::BlankNode(b) => Ok(IriOrBlankNode::BlankNode(b)),
            Object::Literal(l) => Err(RDFError::ExpectedIriOrBlankNodeFoundLiteral {
                literal: l.to_string(),
            }),
            Object::Triple {
                subject,
                predicate,
                object,
            } => Err(RDFError::ExpectedIriOrBlankNodeFoundTriple {
                subject: subject.to_string(),
                predicate: predicate.to_string(),
                object: object.to_string(),
            }),
        }
    }
}

impl From<oxrdf::NamedOrBlankNode> for IriOrBlankNode {
    fn from(value: oxrdf::NamedOrBlankNode) -> Self {
        match value {
            oxrdf::NamedOrBlankNode::NamedNode(iri) => IriOrBlankNode::Iri(iri.into()),
            oxrdf::NamedOrBlankNode::BlankNode(bnode) => {
                IriOrBlankNode::BlankNode(bnode.into_string())
            }
        }
    }
}
