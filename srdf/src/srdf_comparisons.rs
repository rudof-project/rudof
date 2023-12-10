use std::fmt::{Debug, Display};
use std::hash::Hash;

use iri_s::IriS;
use prefixmap::PrefixMapError;

use crate::Object;

pub trait SRDFComparisons {
    type Subject: Debug + Display;
    type IRI: Debug + Display + Hash + Eq + Clone;
    type BNode: Debug + Display;
    type Literal: Debug + Display;
    type Term: Debug + Display;
    type Err: Display;

    fn subject2iri(&self, subject: &Self::Subject) -> Option<Self::IRI>;
    fn subject2bnode(&self, subject: &Self::Subject) -> Option<Self::BNode>;
    fn subject_is_iri(&self, subject: &Self::Subject) -> bool;
    fn subject_is_bnode(&self, subject: &Self::Subject) -> bool;

    fn object2iri(&self, object: &Self::Term) -> Option<Self::IRI>;
    fn object2bnode(&self, object: &Self::Term) -> Option<Self::BNode>;
    fn object2literal(&self, object: &Self::Term) -> Option<Self::Literal>;
    fn object_is_iri(&self, object: &Self::Term) -> bool;
    fn object_is_bnode(&self, object: &Self::Term) -> bool;
    fn object_is_literal(&self, object: &Self::Term) -> bool;

    fn term_as_subject(&self, object: &Self::Term) -> Option<Self::Subject>;
    fn subject_as_term(&self, subject: &Self::Subject) -> Self::Term;

    fn lexical_form(&self, literal: &Self::Literal) -> String;
    fn lang(&self, literal: &Self::Literal) -> Option<String>;
    fn datatype(&self, literal: &Self::Literal) -> Self::IRI;
    
    fn iri_s2iri(iri_s: &IriS) -> Self::IRI;
    fn iri_as_term(iri: Self::IRI) -> Self::Term;

    fn term2object(term: Self::Term) -> Object;
    fn iri2iri_s(iri: Self::IRI) -> IriS;

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError>;

    fn qualify_iri(&self, iri: &Self::IRI) -> String;
    fn qualify_subject(&self, subj: &Self::Subject) -> String ;
    fn qualify_term(&self, subj: &Self::Term) -> String ;

}
