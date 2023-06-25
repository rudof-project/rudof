pub mod bnode;
pub mod lang;
pub mod literal;
pub mod rdf;

use std::{collections::HashSet, fmt::Display};

use async_trait::async_trait;
pub use bag::Bag;
pub use bnode::*;
pub use iri_s::*;
pub use rdf::*;

#[async_trait]
pub trait SRDF {
    type Subject: Display;
    type IRI: Display;
    type BNode: Display;
    type Literal: Display;
    type Term: Display;
    type Err: Display;

    async fn get_predicates_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<Bag<Self::IRI>, Self::Err>;

    async fn get_objects_for_subject_predicate(
        &self,
        subject: &Self::Subject,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err>;

    async fn get_subjects_for_object_predicate(
        &self,
        object: &Self::Term,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Subject>, Self::Err>;

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

    fn iri_from_str(&self, str: String) -> Self::IRI;
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
