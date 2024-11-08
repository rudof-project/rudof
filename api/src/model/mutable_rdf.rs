use super::rdf::Rdf;

pub trait MutableRdf: Rdf {
    fn add_triple(
        &self,
        subject: &Self::Subject,
        predicate: &Self::IRI,
        object: &Self::Term,
    ) -> Result<(), Self::Error>;

    fn remove_triple(
        &self,
        subject: &Self::Subject,
        predicate: &Self::IRI,
        object: &Self::Term,
    ) -> Result<(), Self::Error>;

    fn add_base(&mut self, base: &Self::IRI) -> Result<(), Self::Error>;

    fn add_prefix(&mut self, alias: &str, iri: &Self::IRI) -> Result<(), Self::Error>;
}

pub trait RdfConversion: Rdf {
    fn subject_as_iri(subject: &Self::Subject) -> Option<&Self::IRI>;
    fn subject_as_bnode(subject: &Self::Subject) -> Option<&Self::BNode>;
    fn term_as_iri(term: &Self::Term) -> Option<&Self::IRI>;
    fn term_as_bnode(term: &Self::Term) -> Option<&Self::BNode>;
    fn term_as_literal(term: &Self::Term) -> Option<&Self::Literal>;
    fn term_as_triple(term: &Self::Term) -> Option<&Self::Triple>;
    fn term_as_subject(term: &Self::Term) -> Option<&Self::Subject>;
}

pub trait RdfComparison: Rdf {
    fn subject_is_iri(subject: &Self::Subject) -> bool;
    fn subject_is_bnode(subject: &Self::Subject) -> bool;
    fn term_is_iri(term: &Self::Term) -> bool;
    fn term_is_bnode(term: &Self::Term) -> bool;
    fn term_is_literal(term: &Self::Term) -> bool;
    fn term_is_triple(term: &Self::Term) -> bool;
}
