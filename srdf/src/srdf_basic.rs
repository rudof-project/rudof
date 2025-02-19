use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use iri_s::IriS;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxNamedNode;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use prefixmap::PrefixMap;
use prefixmap::PrefixMapError;

use crate::Object;

pub trait Rdf {
    type Subject: Subject + From<Self::IRI> + From<Self::BNode> + TryFrom<Self::Term>;

    type Term: Term
        + From<Self::Subject>
        + From<Self::IRI>
        + From<Self::BNode>
        + From<Self::Literal>;

    type IRI: Iri + TryFrom<Self::Term>;

    type BNode: BlankNode + TryFrom<Self::Term>;

    type Literal: Literal + TryFrom<Self::Term>;

    type Err: Display;

    /// Returns the RDF subject as an IRI if it is an IRI, None if it isn't
    // fn subject_as_iri(subject: &Self::Subject) -> Option<Self::IRI>; TODO: remove this

    /// Returns the RDF subject as a Blank Node if it is a blank node, None if it isn't
    // fn subject_as_bnode(subject: &Self::Subject) -> Option<Self::BNode>; TODO: remove this

    /// Returns `true` if the subject is an IRI
    fn subject_is_iri(subject: &Self::Subject) -> bool;

    /// Returns `true` if the subject is a Blank Node
    fn subject_is_bnode(subject: &Self::Subject) -> bool;

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
    fn term_as_object(term: &Self::Term) -> Object;

    // TODO: this is removable
    fn object_as_term(obj: &Object) -> Self::Term;

    // TODO: this is removable
    fn object_as_subject(obj: &Object) -> Option<Self::Subject> {
        let term = Self::object_as_term(obj);
        let subject = term.try_into().ok()?;
        Some(subject)
    }

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
    fn term_as_iri_s(term: &Self::Term) -> Option<IriS> {
        let iri_s = match term.clone().try_into() {
            Ok(iri) => Self::iri2iri_s(&iri),
            Err(_) => return None,
        };
        Some(iri_s)
    }

    fn term_is_iri(object: &Self::Term) -> bool;
    fn term_is_bnode(object: &Self::Term) -> bool;
    fn term_is_literal(object: &Self::Term) -> bool;

    // fn term_as_subject(object: &Self::Term) -> Option<Self::Subject>;

    // fn subject_as_term(subject: &Self::Subject) -> Self::Term;

    // TODO: this is removable
    fn subject_as_object(subject: &Self::Subject) -> Object {
        let term = subject.clone().into();
        Self::term_as_object(&term)
    }

    fn lexical_form(literal: &Self::Literal) -> &str;
    fn lang(literal: &Self::Literal) -> Option<String>;
    fn datatype(literal: &Self::Literal) -> Self::IRI;

    fn datatype_str(literal: &Self::Literal) -> String {
        let iri = Self::datatype(literal);
        Self::iri2iri_s(&iri).to_string()
    }

    // TODO: this is removable
    fn iri_s2iri(iri_s: &IriS) -> Self::IRI;

    // TODO: this is removable
    fn term_s2term(term: &OxTerm) -> Self::Term;

    // TODO: this is removable
    fn bnode_id2bnode(id: &str) -> Self::BNode;

    // TODO: this is removable
    fn iri_s2subject(iri_s: &IriS) -> Self::Subject {
        todo!()
    }

    // TODO: this is removable
    fn iri_s2term(iri_s: &IriS) -> Self::Term {
        todo!()
    }

    // TODO: this is removable
    fn bnode_id2term(id: &str) -> Self::Term {
        todo!()
    }

    // TODO: this is removable
    fn bnode_id2subject(id: &str) -> Self::Subject {
        todo!()
    }

    // fn iri_as_term(iri: Self::IRI) -> Self::Term;

    // fn iri_as_subject(iri: Self::IRI) -> Self::Subject;

    // fn bnode_as_term(bnode: Self::BNode) -> Self::Term;

    // fn bnode_as_subject(bnode: Self::BNode) -> Self::Subject;

    // TODO: this is removable
    fn iri2iri_s(iri: &Self::IRI) -> IriS;

    fn qualify_iri(&self, iri: &Self::IRI) -> String;
    fn qualify_subject(&self, subj: &Self::Subject) -> String;
    fn qualify_term(&self, term: &Self::Term) -> String;

    fn prefixmap(&self) -> Option<PrefixMap>;

    /// Resolves a a prefix and a local name and obtains the corresponding full `IriS`
    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError>;
}

pub trait Subject: Debug + Display + PartialEq + Clone + Eq + Hash {}

impl Subject for OxSubject {}

pub trait Iri: Debug + Display + Hash + Eq + Clone {
    fn as_str(&self) -> &str;
}

impl Iri for OxNamedNode {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

pub trait Term: Debug + Clone + Display + PartialEq + Eq + Hash {}

impl Term for OxTerm {}

pub trait Literal: Debug + Display + PartialEq + Eq + Hash {
    fn as_str(&self) -> &str;

    fn as_bool(&self) -> Option<bool> {
        match self.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    fn as_integer(&self) -> Option<isize> {
        match self.as_str().parse() {
            Ok(n) => Some(n),
            _ => None,
        }
    }
}

impl Literal for OxLiteral {
    fn as_str(&self) -> &str {
        self.value()
    }
}

pub trait BlankNode: Debug + Display + PartialEq {}

impl BlankNode for OxBlankNode {}
