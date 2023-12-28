use std::fmt::{Debug, Display};
use std::hash::Hash;

use iri_s::IriS;
use prefixmap::{PrefixMapError, PrefixMap};

use crate::Object;

/// Trait that contains comparisons and conversions between nodes in RDF graphs
pub trait SRDFComparisons {
    type Subject: Debug + Display;
    type IRI: Debug + Display + Hash + Eq + Clone;
    type BNode: Debug + Display + PartialEq;
    type Literal: Debug + Display + PartialEq;
    type Term: Debug + Clone + Display + PartialEq;
    type Err: Display;

    fn subject_as_iri(subject: &Self::Subject) -> Option<Self::IRI>;
    fn subject_as_bnode(subject: &Self::Subject) -> Option<Self::BNode>;
    fn subject_is_iri(subject: &Self::Subject) -> bool;
    fn subject_is_bnode(subject: &Self::Subject) -> bool;

    fn object_as_iri(object: &Self::Term) -> Option<Self::IRI>;
    fn object_as_bnode(object: &Self::Term) -> Option<Self::BNode>;
    fn object_as_literal(object: &Self::Term) -> Option<Self::Literal>;
    fn object_as_boolean(object: &Self::Term) -> Option<bool> {
        let literal = Self::object_as_literal(object)?;
        Self::literal_as_boolean(&literal)
    }

    fn literal_as_boolean(literal: &Self::Literal) -> Option<bool> {
        match &Self::datatype_str(&literal) {
            RDF_BOOLEAN => match Self::lexical_form(literal) {
               "true" => Some(true),
               "false" => Some(false),
               _ => None
             },
            _ => None
        } 
    }

    fn object_is_iri(object: &Self::Term) -> bool;
    fn object_is_bnode(object: &Self::Term) -> bool;
    fn object_is_literal(object: &Self::Term) -> bool;

    fn term_as_subject(object: &Self::Term) -> Option<Self::Subject>;

    fn subject_as_term(subject: &Self::Subject) -> Self::Term;

    fn lexical_form(literal: &Self::Literal) -> &str;
    fn lang(literal: &Self::Literal) -> Option<String>;
    fn datatype(literal: &Self::Literal) -> Self::IRI;
    
    fn datatype_str(literal: &Self::Literal) -> String {
        let iri = Self::datatype(literal);
        Self::iri2iri_s(&iri).to_string()
    }

    fn iri_s2iri(iri_s: &IriS) -> Self::IRI;
    fn iri_s2subject(iri_s: &IriS) -> Self::Subject {
        Self::iri_as_subject(Self::iri_s2iri(iri_s))
    }
    fn iri_s2term(iri_s: &IriS) -> Self::Term {
        Self::iri_as_term(Self::iri_s2iri(iri_s))
    }

    fn iri_as_term(iri: Self::IRI) -> Self::Term;
    fn iri_as_subject(iri: Self::IRI) -> Self::Subject;

    fn term_as_object(term: &Self::Term) -> Object;
    fn iri2iri_s(iri: &Self::IRI) -> IriS;

    fn qualify_iri(&self, iri: &Self::IRI) -> String;
    fn qualify_subject(&self, subj: &Self::Subject) -> String;
    fn qualify_term(&self, subj: &Self::Term) -> String;

    fn prefixmap(&self) -> Option<PrefixMap>;
    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError>;

}
