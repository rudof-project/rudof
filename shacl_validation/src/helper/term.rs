use std::{fmt, str::FromStr};

use iri_s::IriS;
use oxigraph::model::Term as OxTerm;
use oxrdf::vocab::xsd::{BOOLEAN, DECIMAL, DOUBLE, INTEGER, STRING};
use prefixmap::IriRef;
use srdf::{lang::Lang, numeric_literal::NumericLiteral, Object};

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum Term {
    IRI(IriS),
    BlankNode(String),
    Literal(LiteralContent),
}

impl Term {
    pub(crate) fn is_iri(&self) -> bool {
        match self {
            Term::IRI(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_blank_node(&self) -> bool {
        match self {
            Term::BlankNode(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_literal(&self) -> bool {
        match self {
            Term::Literal(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_triple(&self) -> bool {
        false
    }
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum LiteralContent {
    String(String, Option<Lang>),
    Datatype(String, IriRef),
}

impl LiteralContent {
    pub(crate) fn datatype(&self) -> IriRef {
        match self {
            LiteralContent::String(_, _) => IriRef::from_str(STRING.as_str()).unwrap(),
            LiteralContent::Datatype(_, iri) => iri.to_owned(),
        }
    }

    pub(crate) fn lang(&self) -> &Option<Lang> {
        match self {
            LiteralContent::String(_, lang) => lang,
            LiteralContent::Datatype(_, _) => &None,
        }
    }
}

impl From<&Term> for Object {
    fn from(value: &Term) -> Self {
        match value {
            Term::IRI(iri) => Object::iri(iri.to_owned()),
            Term::BlankNode(str) => Object::bnode(str.to_string()),
            Term::Literal(literal) => Object::literal(match literal {
                LiteralContent::String(value, lang) => match lang.to_owned() {
                    Some(lang) => srdf::literal::Literal::lang_str(value, lang),
                    None => srdf::literal::Literal::str(value),
                },
                LiteralContent::Datatype(value, datatype) => {
                    srdf::literal::Literal::datatype(value, datatype)
                }
            }),
        }
    }
}

impl Into<Term> for Object {
    fn into(self) -> Term {
        match self {
            Object::Iri(iri) => Term::IRI(iri),
            Object::BlankNode(str) => Term::BlankNode(str),
            Object::Literal(literal) => match literal {
                srdf::literal::Literal::StringLiteral { lexical_form, lang } => {
                    Term::Literal(LiteralContent::String(lexical_form, lang))
                }
                srdf::literal::Literal::DatatypeLiteral {
                    lexical_form,
                    datatype,
                } => Term::Literal(LiteralContent::Datatype(lexical_form, datatype)),
                srdf::literal::Literal::NumericLiteral(value) => {
                    Term::Literal(LiteralContent::Datatype(
                        value.to_string(),
                        match value {
                            NumericLiteral::Integer(_) => {
                                IriRef::from_str(INTEGER.as_str()).unwrap()
                            }
                            NumericLiteral::Decimal(_) => {
                                IriRef::from_str(DECIMAL.as_str()).unwrap()
                            }
                            NumericLiteral::Double(_) => IriRef::from_str(DOUBLE.as_str()).unwrap(),
                        },
                    ))
                }
                srdf::literal::Literal::BooleanLiteral(value) => {
                    Term::Literal(LiteralContent::Datatype(
                        value.to_string(),
                        IriRef::from_str(BOOLEAN.as_str()).unwrap(),
                    ))
                }
            },
        }
    }
}

impl From<OxTerm> for Term {
    fn from(value: OxTerm) -> Self {
        match value {
            OxTerm::NamedNode(node) => Term::IRI(IriS::new_unchecked(node.as_str())),
            OxTerm::BlankNode(node) => Term::BlankNode(node.into_string()),
            OxTerm::Literal(literal) => match literal.destruct() {
                (s, None, None) => Term::Literal(LiteralContent::String(s, None)),
                (s, None, Some(language)) => {
                    Term::Literal(LiteralContent::String(s, Some(Lang::new(&language))))
                }
                (value, Some(datatype), None) => Term::Literal(LiteralContent::Datatype(
                    value,
                    IriRef::from_str(&datatype.into_string()).unwrap(),
                )),
                _ => unreachable!(),
            },
            OxTerm::Triple(_) => todo!(),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::IRI(iri) => write!(f, "{iri}"),
            Term::BlankNode(bnode) => write!(f, "_{bnode}"),
            Term::Literal(lit) => write!(f, "{lit}"),
        }
    }
}

impl fmt::Display for LiteralContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralContent::String(value, lang) => match lang {
                Some(lang) => write!(f, "\"{value}\"{lang}"),
                None => write!(f, "\"{value}\""),
            },
            LiteralContent::Datatype(value, datatype) => write!(f, "\"{value}\"^^<{datatype}>"),
        }
    }
}
