use std::fmt::Display;
use std::hash::Hash;

pub trait SRDFComparisons {
    type Subject: Display ;
    type IRI: Display + Hash + Eq ;
    type BNode: Display ;
    type Literal: Display ;
    type Term: Display ;
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

    fn lexical_form(&self, literal: &Self::Literal) -> String;
    fn lang(&self, literal: &Self::Literal) -> Option<String>;
    fn datatype(&self, literal: &Self::Literal) -> Self::IRI;

    fn iri_from_str(str: &str) -> Result<Self::IRI, Self::Err>;
    fn iri_as_term(iri: Self::IRI) -> Self::Term ;
}
