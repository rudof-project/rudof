use std::{collections::HashSet, fmt::Display};
use std::hash::Hash;

use crate::SRDFComparisons;


pub trait SRDF: SRDFComparisons {
/*    type Subject: Display ;
    type IRI: Display + Hash + Eq ;
    type BNode: Display ;
    type Literal: Display ;
    type Term: Display ;
    type Err: Display; */

    fn get_predicates_for_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashSet<Self::IRI>, Self::Err>;

    fn get_objects_for_subject_predicate(
        &self,
        subject: &Self::Subject,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err>;

    fn get_subjects_for_object_predicate(
        &self,
        object: &Self::Term,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Subject>, Self::Err>;

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use iri_s::*;

    #[test]
    fn check_2_iris() {
        let iri1: IriS = IriS::from_str("http://example.org/iri").unwrap();
        let iri2 = IriS::from_str("http://example.org/iri").unwrap();
        assert_eq!(iri1, iri2);
    }
}
