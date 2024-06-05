use std::{collections::HashSet, fmt::Display};

use async_trait::async_trait;
use std::hash::Hash;

#[async_trait]
pub trait AsyncSRDF {
    type Subject: Display + Sync + Send;
    type IRI: Display + Hash + Eq + Sync + Send;
    type BNode: Display + Sync + Send;
    type Literal: Display + Sync + Send;
    type Term: Display + Sync + Send;
    type Err: Display;

    async fn get_predicates_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashSet<Self::IRI>, Self::Err>;

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
}

#[cfg(test)]
mod tests {
    use iri_s::*;
    use std::str::FromStr;

    #[test]
    fn check_2_iris() {
        let iri1: IriS = IriS::from_str("http://example.org/iri").unwrap();
        let iri2 = IriS::from_str("http://example.org/iri").unwrap();
        assert_eq!(iri1, iri2);
    }
}
