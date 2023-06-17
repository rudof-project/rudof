pub mod bnode;
pub mod lang;
pub mod literal;
pub mod rdf;

pub use bnode::*;
pub use iri_s::*;
pub use rdf::*;

pub trait SRDF {
    type Subject;
    type IRI ;
    type BNode;
    type Literal;
    type Term ;

    fn get_predicates_subject(&self, subject: &Self::Subject) -> Vec<Self::IRI> ; 
    fn get_objects_for_subject_predicate(&self, subject: &Self::Subject, pred: &Self::IRI) -> Vec<Self::Term> ;
    fn get_subjects_for_object_predicate(&self, object: &Self::Term, pred: &Self::IRI) -> Vec<Self::Subject> ;

    fn subject2iri(&self, subject:&Self::Subject) -> Option<Self::IRI>;
    fn subject2bnode(&self, subject:&Self::Subject) -> Option<Self::BNode>;
    fn subject_is_iri(&self, subject:&Self::Subject) -> bool;
    fn subject_is_bnode(&self, subject:&Self::Subject) -> bool;

    fn object2iri(&self, object:&Self::Term) -> Option<Self::IRI>;
    fn object2bnode(&self, object:&Self::Term) -> Option<Self::BNode>;
    fn object2literal(&self, object:&Self::Term) -> Option<Self::Literal>;
    fn object_is_iri(&self, object:&Self::Term) -> bool;
    fn object_is_bnode(&self, object:&Self::Term) -> bool;
    fn object_is_literal(&self, object:&Self::Term) -> bool;

    fn lexical_form(&self, literal: &Self::Literal) -> String;
    fn lang(&self, literal: &Self::Literal) -> Option<String>;
    fn datatype(&self, literal: &Self::Literal) -> Self::IRI;

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn check_2_iris() {
        let iri1: IriS = IriS::from_str("http://example.org/iri").unwrap();
        let iri2 = IriS::from_str("http://example.org/iri").unwrap();
        assert_eq!(iri1, iri2);
    }
}
