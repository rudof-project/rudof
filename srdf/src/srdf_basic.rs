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
        + From<crate::literal::Literal>; // TODO: can we use From<&str>?

    type Triple: Triple<Self::Subject, Self::IRI, Self::Term>;

    type Err: Display;

    // fn subject_as_iri(subject: &Self::Subject) -> Option<Self::IRI>; TODO: remove this

    // fn subject_as_bnode(subject: &Self::Subject) -> Option<Self::BNode>; TODO: remove this

    /// Returns `true` if the subject is an IRI
    // fn subject_is_iri(subject: &Self::Subject) -> bool;

    /// Returns `true` if the subject is a Blank Node
    // fn subject_is_bnode(subject: &Self::Subject) -> bool;

    // fn term_as_iri(object: &Self::Term) -> Option<Self::IRI>; TODO: remove this

    // fn term_as_bnode(object: &Self::Term) -> Option<Self::BNode>; TODO: remove this
    // fn term_as_literal(object: &Self::Term) -> Option<Self::Literal>; TODO: remove this

    // TODO: this is removable
    // fn term_as_boolean(term: &Self::Term) -> Option<bool> {
    //     let literal = term.clone().try_into().ok()?;
    //     Self::literal_as_boolean(&literal)
    // }

    // TODO: this is removable
    // fn term_as_integer(term: &Self::Term) -> Option<isize> {
    //     let literal = term.clone().try_into().ok()?;
    //     Self::literal_as_integer(&literal)
    // }

    // TODO: this is removable
    // fn term_as_string(term: &Self::Term) -> Option<String> {
    //     let literal = term.clone().try_into().ok()?;
    //     Self::literal_as_string(&literal)
    // }

    // TODO: this is removable
    // fn term_as_object(term: &Self::Term) -> Object;

    // TODO: this is removable
    // fn object_as_term(obj: &Object) -> Self::Term;

    // TODO: this is removable
    // fn object_as_subject(obj: &Object) -> Option<Self::Subject> {
    //     let term = Self::object_as_term(obj);
    //     let subject = term.try_into().ok()?;
    //     Some(subject)
    // }

    // TODO: this is removable
    // fn subject_as_object(subject: &Self::Subject) -> Object {
    //     let term = subject.clone().into();
    //     Self::term_as_object(&term)
    // }

    // TODO: this is removable
    // fn literal_as_boolean(literal: &Self::Literal) -> Option<bool> {
    //     match Self::lexical_form(literal) {
    //         "true" => Some(true),
    //         "false" => Some(false),
    //         _ => None,
    //     }
    // }

    // fn literal_as_integer(literal: &Self::Literal) -> Option<isize> {
    //     match Self::lexical_form(literal).parse() {
    //         Ok(n) => Some(n),
    //         _ => None,
    //     }
    // }

    // fn literal_as_string(literal: &Self::Literal) -> Option<String> {
    //     Some(Self::lexical_form(literal).to_string())
    // }

    // TODO: this is removable
    // fn term_as_iri_s(term: &Self::Term) -> Option<IriS> {
    //     let iri_s = match term.clone().try_into() {
    //         Ok(iri) => Self::iri2iri_s(&iri),
    //         Err(_) => return None,
    //     };
    //     Some(iri_s)
    // }

    // TODO: this is removable
    // fn iri2iri_s(iri: &Self::IRI) -> IriS;

    // fn term_is_iri(object: &Self::Term) -> bool;
    // fn term_is_bnode(object: &Self::Term) -> bool;
    // fn term_is_literal(object: &Self::Term) -> bool;

    // fn term_as_subject(object: &Self::Term) -> Option<Self::Subject>;

    // fn subject_as_term(subject: &Self::Subject) -> Self::Term;

    // fn lexical_form(literal: &Self::Literal) -> &str;
    // fn lang(literal: &Self::Literal) -> Option<String>;
    // fn datatype(literal: &Self::Literal) -> Self::IRI;

    // fn datatype_str(literal: &Self::Literal) -> String {
    //     let iri = Self::datatype(literal);
    //     Self::iri2iri_s(&iri).to_string()
    // }

    // TODO: this is removable
    // fn iri_s2iri(iri_s: &IriS) -> Self::IRI;

    // TODO: this is removable
    // fn term_s2term(term: &OxTerm) -> Self::Term;

    // TODO: this is removable
    // fn bnode_id2bnode(id: &str) -> Self::BNode;

    // TODO: this is removable
    // fn iri_s2subject(iri_s: &IriS) -> Self::Subject {
    //     todo!()
    // }

    // TODO: this is removable
    // fn iri_s2term(iri_s: &IriS) -> Self::Term {
    //     todo!()
    // }

    // TODO: this is removable
    // fn bnode_id2term(id: &str) -> Self::Term {
    //     todo!()
    // }

    // TODO: this is removable
    // fn bnode_id2subject(id: &str) -> Self::Subject {
    //     todo!()
    // }

    // fn iri_as_term(iri: Self::IRI) -> Self::Term;

    // fn iri_as_subject(iri: Self::IRI) -> Self::Subject;

    // fn bnode_as_term(bnode: Self::BNode) -> Self::Term;

    // fn bnode_as_subject(bnode: Self::BNode) -> Self::Subject;

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

impl Matcher<OxSubject> for OxSubject {
    fn value(&self) -> Option<OxSubject> {
        Some(self.clone())
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

impl Matcher<OxNamedNode> for OxNamedNode {
    fn value(&self) -> Option<OxNamedNode> {
        Some(self.clone())
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

impl Matcher<OxTerm> for OxTerm {
    fn value(&self) -> Option<OxTerm> {
        Some(self.clone())
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

    fn as_literal(&self) -> crate::literal::Literal {
        if let Some(bool) = self.as_bool() {
            crate::literal::Literal::boolean(bool)
        } else if let Some(int) = self.as_integer() {
            crate::literal::Literal::integer(int)
        } else if let Some(decimal) = self.as_double() {
            crate::literal::Literal::double(decimal)
        } else if let Some(decimal) = self.as_decimal() {
            crate::literal::Literal::decimal(decimal)
        } else if let Some(lang) = self.lang() {
            crate::literal::Literal::lang_str(self.lexical_form(), crate::lang::Lang::new(lang))
        } else {
            crate::literal::Literal::str(self.lexical_form())
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

    fn subj(&self) -> S;
    fn pred(&self) -> P;
    fn obj(&self) -> O;

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

    fn subj(&self) -> OxSubject {
        self.subject.clone()
    }

    fn pred(&self) -> OxNamedNode {
        self.predicate.clone()
    }

    fn obj(&self) -> OxTerm {
        self.object.clone()
    }

    fn into_components(self) -> (OxSubject, OxNamedNode, OxTerm) {
        (self.subject, self.predicate, self.object)
    }
}
