use std::fmt::{Debug, Display};

use crate::IriOrBlankNode;
use crate::RDFError;
use crate::SLiteral;
use crate::lang::Lang;
use crate::numeric_literal::NumericLiteral;
use crate::triple::Triple;
use iri_s::IriS;
use prefixmap::IriRef;
use serde::{Deserialize, Serialize};
use tracing::trace;

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

    pub fn parse(str: &str, base: Option<&str>) -> Result<Object, RDFError> {
        if let Some(bnode_id) = str.strip_prefix("_:") {
            trace!("Parsing blank node id: {bnode_id} from str: {str}");
            Ok(Object::bnode(bnode_id.to_string()))
        } else if str.starts_with('"') {
            trace!("Pending parsing literal from str: {str}");
            todo!()
        } else {
            let iri = IriS::from_str_base(str, base).map_err(|e| RDFError::ParsingIri {
                iri: str.to_string(),
                error: e.to_string(),
            })?;
            Ok(Object::iri(iri))
        }
    }

    pub fn lang(&self) -> Option<&Lang> {
        match self {
            Object::Literal(SLiteral::StringLiteral {
                lang: Some(lang), ..
            }) => Some(lang),
            _ => None,
        }
    }

    pub fn show_qualified(
        &self,
        prefixmap: &prefixmap::PrefixMap,
    ) -> Result<String, prefixmap::PrefixMapError> {
        match self {
            Object::Iri(iri) => Ok(prefixmap.qualify(iri)),
            Object::BlankNode(bnode) => Ok(format!("_:{bnode}")),
            Object::Literal(lit) => Ok(lit.to_string()),
            Object::Triple {
                subject,
                predicate,
                object,
            } => Ok(format!(
                "<< {} {} {} >>",
                subject.show_qualified(prefixmap)?,
                prefixmap.qualify(predicate),
                object.show_qualified(prefixmap)?
            )),
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
            Object::BlankNode(bnode) => write!(f, "_:{bnode}"),
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
        Some(self.cmp(other))
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Object::Iri(a), Object::Iri(b)) => a.cmp(b),
            (Object::BlankNode(a), Object::BlankNode(b)) => a.cmp(b),
            (Object::Literal(a), Object::Literal(b)) => a.cmp(b),
            (Object::Iri(_), _) => std::cmp::Ordering::Less,
            (Object::BlankNode(_), Object::Iri(_)) => std::cmp::Ordering::Greater,
            (Object::BlankNode(_), Object::Literal(_)) => std::cmp::Ordering::Less,
            (Object::Literal(_), _) => std::cmp::Ordering::Greater,
            (
                Object::BlankNode(_),
                Object::Triple {
                    subject: _,
                    predicate: _,
                    object: _,
                },
            ) => todo!(),
            (
                Object::Triple {
                    subject: _,
                    predicate: _,
                    object: _,
                },
                Object::Iri(_iri_s),
            ) => todo!(),
            (
                Object::Triple {
                    subject: _,
                    predicate: _,
                    object: _,
                },
                Object::BlankNode(_),
            ) => todo!(),
            (
                Object::Triple {
                    subject: _,
                    predicate: _,
                    object: _,
                },
                Object::Literal(_sliteral),
            ) => todo!(),
            (
                Object::Triple {
                    subject: _subject1,
                    predicate: _predicate1,
                    object: _object1,
                },
                Object::Triple {
                    subject: _subject2,
                    predicate: _predicate2,
                    object: _object2,
                },
            ) => todo!(),
        }
    }
}
