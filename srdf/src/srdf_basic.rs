use std::fmt::{Debug, Display};
use std::hash::Hash;

use iri_s::IriS;
use oxrdf::Term as OxTerm;
use prefixmap::{PrefixMap, PrefixMapError};

use crate::Object;

/// Types that implement this trait contain basic comparisons and conversions between nodes in RDF graphs
///
pub trait SRDFBasic {
    /// RDF subjects
    type Subject: Debug + Display + PartialEq + Clone + Eq + Hash;

    /// RDF predicates
    type IRI: Debug + Display + Hash + Eq + Clone;

    /// Blannk nodes
    type BNode: Debug + Display + PartialEq;

    /// RDF Literals
    type Literal: Debug + Display + PartialEq;

    /// RDF terms
    type Term: Debug + Clone + Display + PartialEq + Eq + Hash;

    /// RDF errors
    type Err: Display;

    /// Returns the RDF subject as an IRI if it is an IRI, None if it isn't
    fn subject_as_iri(subject: &Self::Subject) -> Option<Self::IRI>;

    /// Returns the RDF subject as a Blank Node if it is a blank node, None if it isn't
    fn subject_as_bnode(subject: &Self::Subject) -> Option<Self::BNode>;

    /// Returns `true` if the subject is an IRI
    fn subject_is_iri(subject: &Self::Subject) -> bool;

    /// Returns `true` if the subject is a Blank Node
    fn subject_is_bnode(subject: &Self::Subject) -> bool;

    fn term_as_iri(object: &Self::Term) -> Option<Self::IRI>;
    fn term_as_bnode(object: &Self::Term) -> Option<Self::BNode>;
    fn term_as_literal(object: &Self::Term) -> Option<Self::Literal>;
    fn term_as_boolean(object: &Self::Term) -> Option<bool> {
        let literal = Self::term_as_literal(object)?;
        Self::literal_as_boolean(&literal)
    }
    fn term_as_object(term: &Self::Term) -> Object;

    fn object_as_term(obj: &Object) -> Self::Term;
    fn object_as_subject(obj: &Object) -> Option<Self::Subject> {
        let term = Self::object_as_term(obj);
        Self::term_as_subject(&term)
    }

    fn literal_as_boolean(literal: &Self::Literal) -> Option<bool> {
        match Self::lexical_form(literal) {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    fn literal_as_integer(literal: &Self::Literal) -> Option<isize> {
        match Self::lexical_form(literal).parse() {
            Ok(n) => Some(n),
            _ => None,
        }
    }

    fn literal_as_string(literal: &Self::Literal) -> Option<String> {
        Some(Self::lexical_form(literal).to_string())
    }

    fn term_as_integer(term: &Self::Term) -> Option<isize> {
        Self::term_as_literal(term).and_then(|l| Self::literal_as_integer(&l))
    }

    fn term_as_string(term: &Self::Term) -> Option<String> {
        Self::term_as_literal(term).and_then(|l| Self::literal_as_string(&l))
    }

    fn term_is_iri(object: &Self::Term) -> bool;
    fn term_is_bnode(object: &Self::Term) -> bool;
    fn term_is_literal(object: &Self::Term) -> bool;

    fn term_as_subject(object: &Self::Term) -> Option<Self::Subject>;

    fn subject_as_term(subject: &Self::Subject) -> Self::Term;

    fn subject_as_object(subject: &Self::Subject) -> Object {
        Self::term_as_object(&Self::subject_as_term(subject))
    }

    fn lexical_form(literal: &Self::Literal) -> &str;
    fn lang(literal: &Self::Literal) -> Option<String>;
    fn datatype(literal: &Self::Literal) -> Self::IRI;

    fn datatype_str(literal: &Self::Literal) -> String {
        let iri = Self::datatype(literal);
        Self::iri2iri_s(&iri).to_string()
    }

    fn iri_s2iri(iri_s: &IriS) -> Self::IRI;

    fn term_s2term(term: &OxTerm) -> Self::Term;
    fn bnode_id2bnode(id: &str) -> Self::BNode;

    fn iri_s2subject(iri_s: &IriS) -> Self::Subject {
        Self::iri_as_subject(Self::iri_s2iri(iri_s))
    }
    fn iri_s2term(iri_s: &IriS) -> Self::Term {
        Self::iri_as_term(Self::iri_s2iri(iri_s))
    }

    fn bnode_id2term(id: &str) -> Self::Term {
        Self::bnode_as_term(Self::bnode_id2bnode(id))
    }

    fn bnode_id2subject(id: &str) -> Self::Subject {
        Self::bnode_as_subject(Self::bnode_id2bnode(id))
    }

    fn iri_as_term(iri: Self::IRI) -> Self::Term;
    fn iri_as_subject(iri: Self::IRI) -> Self::Subject;

    fn bnode_as_term(bnode: Self::BNode) -> Self::Term;
    fn bnode_as_subject(bnode: Self::BNode) -> Self::Subject;

    fn iri2iri_s(iri: &Self::IRI) -> IriS;

    fn qualify_iri(&self, iri: &Self::IRI) -> String;
    fn qualify_subject(&self, subj: &Self::Subject) -> String;
    fn qualify_term(&self, subj: &Self::Term) -> String;

    fn prefixmap(&self) -> Option<PrefixMap>;
    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError>;
}
