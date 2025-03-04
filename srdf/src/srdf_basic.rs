use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use iri_s::IriS;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxNamedNode;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;
use prefixmap::PrefixMap;
use prefixmap::PrefixMapError;
use rust_decimal::Decimal;

use crate::lang::Lang;
use crate::literal::Literal as SRDFLiteral;
use crate::matcher::Matcher;
use crate::Object;

pub trait Rdf: Sized {
    type Subject: Subject
        + From<Self::IRI>
        + From<Self::BNode>
        + From<IriS>
        + TryFrom<Self::Term>
        + TryFrom<Object>
        + Matcher<Self::Subject>;

    type IRI: Iri + From<IriS> + TryFrom<Self::Term> + Matcher<Self::IRI>;

    type Term: Term
        + From<Self::Subject>
        + From<Self::IRI>
        + From<Self::BNode>
        + From<Self::Literal>
        + From<IriS>
        + From<Object>
        + Into<Object>
        + Matcher<Self::Term>;

    type BNode: BlankNode + TryFrom<Self::Term>;

    type Literal: Literal
        + From<bool>
        + From<String>
        + From<i128>
        + From<f64>
        + TryFrom<Self::Term>
        + From<SRDFLiteral>;

    type Triple: Triple<Self::Subject, Self::IRI, Self::Term>;

    type Err: std::error::Error + 'static;

    fn qualify_iri(&self, iri: &Self::IRI) -> String;
    fn qualify_subject(&self, subj: &Self::Subject) -> String;
    fn qualify_term(&self, term: &Self::Term) -> String;

    fn prefixmap(&self) -> Option<PrefixMap>;

    /// Resolves a a prefix and a local name and obtains the corresponding full `IriS`
    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError>;
}

#[derive(PartialEq)]
pub enum TermKind {
    Iri,
    BlankNode,
    Literal,
    Triple,
}

pub trait Subject: Debug + Display + PartialEq + Clone + Eq + Hash {
    fn kind(&self) -> TermKind;

    fn is_iri(&self) -> bool {
        self.kind() == TermKind::Iri
    }

    fn is_blank_node(&self) -> bool {
        self.kind() == TermKind::BlankNode
    }

    fn is_triple(&self) -> bool {
        self.kind() == TermKind::Triple
    }
}

impl Subject for OxSubject {
    fn kind(&self) -> TermKind {
        match self {
            OxSubject::NamedNode(_) => TermKind::Iri,
            OxSubject::BlankNode(_) => TermKind::BlankNode,
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => TermKind::Triple,
        }
    }
}

pub trait Iri: Debug + Display + Hash + Eq + Clone {
    fn as_str(&self) -> &str;
}

impl Iri for OxNamedNode {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

pub trait Term: Debug + Clone + Display + PartialEq + Eq + Hash {
    fn kind(&self) -> TermKind;

    fn is_iri(&self) -> bool {
        self.kind() == TermKind::Iri
    }

    fn is_blank_node(&self) -> bool {
        self.kind() == TermKind::BlankNode
    }

    fn is_literal(&self) -> bool {
        self.kind() == TermKind::Literal
    }

    fn is_triple(&self) -> bool {
        self.kind() == TermKind::Triple
    }
}

impl Term for OxTerm {
    fn kind(&self) -> TermKind {
        match self {
            OxTerm::NamedNode(_) => TermKind::Iri,
            OxTerm::BlankNode(_) => TermKind::BlankNode,
            OxTerm::Literal(_) => TermKind::Literal,
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => TermKind::Triple,
        }
    }
}

pub trait Literal: Debug + Clone + Display + PartialEq + Eq + Hash {
    fn lexical_form(&self) -> &str;

    fn lang(&self) -> Option<&str>;

    fn datatype(&self) -> &str;

    fn as_bool(&self) -> Option<bool> {
        match self.lexical_form() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    fn as_integer(&self) -> Option<isize> {
        match self.lexical_form().parse() {
            Ok(n) => Some(n),
            _ => None,
        }
    }

    fn as_double(&self) -> Option<f64> {
        match self.lexical_form().parse() {
            Ok(n) => Some(n),
            _ => None,
        }
    }

    fn as_decimal(&self) -> Option<Decimal> {
        match self.lexical_form().parse() {
            Ok(n) => Some(n),
            _ => None,
        }
    }

    fn as_literal(&self) -> SRDFLiteral {
        if let Some(bool) = self.as_bool() {
            SRDFLiteral::boolean(bool)
        } else if let Some(int) = self.as_integer() {
            SRDFLiteral::integer(int)
        } else if let Some(decimal) = self.as_double() {
            SRDFLiteral::double(decimal)
        } else if let Some(decimal) = self.as_decimal() {
            SRDFLiteral::decimal(decimal)
        } else if let Some(lang) = self.lang() {
            SRDFLiteral::lang_str(self.lexical_form(), Lang::new_unchecked(lang))
        } else {
            SRDFLiteral::str(self.lexical_form())
        }
    }
}

impl Literal for OxLiteral {
    fn lexical_form(&self) -> &str {
        self.value()
    }

    fn lang(&self) -> Option<&str> {
        self.language()
    }

    fn datatype(&self) -> &str {
        self.datatype().as_str()
    }
}

pub trait BlankNode: Debug + Display + PartialEq {
    fn new(id: impl Into<String>) -> Self;
    fn id(&self) -> &str;
}

impl BlankNode for OxBlankNode {
    fn new(id: impl Into<String>) -> Self {
        OxBlankNode::new_unchecked(id)
    }

    fn id(&self) -> &str {
        self.as_str()
    }
}

pub trait Triple<S, P, O>: Debug + Clone + Display
where
    S: Subject,
    P: Iri,
    O: Term,
{
    fn new(subj: impl Into<S>, pred: impl Into<P>, obj: impl Into<O>) -> Self;

    fn subj(&self) -> &S;
    fn pred(&self) -> &P;
    fn obj(&self) -> &O;

    fn into_components(self) -> (S, P, O);

    fn into_subject(self) -> S {
        self.into_components().0
    }

    fn into_predicate(self) -> P {
        self.into_components().1
    }

    fn into_object(self) -> O {
        self.into_components().2
    }
}

impl Triple<OxSubject, OxNamedNode, OxTerm> for OxTriple {
    fn new(
        subj: impl Into<OxSubject>,
        pred: impl Into<OxNamedNode>,
        obj: impl Into<OxTerm>,
    ) -> Self {
        OxTriple::new(subj, pred, obj)
    }

    fn subj(&self) -> &OxSubject {
        &self.subject
    }

    fn pred(&self) -> &OxNamedNode {
        &self.predicate
    }

    fn obj(&self) -> &OxTerm {
        &self.object
    }

    fn into_components(self) -> (OxSubject, OxNamedNode, OxTerm) {
        (self.subject, self.predicate, self.object)
    }
}
