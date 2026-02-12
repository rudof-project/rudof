use std::fmt::Display;

use iri_s::IriS;
use serde::{Deserialize, Serialize};

use crate::{Object, RDFError};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
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

    pub fn show_qualified(&self, prefixmap: &prefixmap::PrefixMap) -> String {
        match self {
            IriOrBlankNode::BlankNode(bnode) => format!("_:{bnode}"),
            IriOrBlankNode::Iri(iri) => prefixmap.qualify(iri),
        }
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
            Object::Literal(l) => Err(RDFError::ExpectedIriOrBlankNodeFoundLiteral { literal: l.to_string() }),
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
            oxrdf::NamedOrBlankNode::BlankNode(bnode) => IriOrBlankNode::BlankNode(bnode.into_string()),
        }
    }
}
