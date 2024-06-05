use std::collections::{HashMap, HashSet};
//use std::hash::Hash;

use crate::{SRDFBasic, Triple};

type ListOfIriAndTerms<I, T> = Vec<(I, HashSet<T>)>;
type HasMapOfIriAndItem<I, T> = HashMap<I, HashSet<T>>;

type OutgoingArcs<I, T> = (HasMapOfIriAndItem<I, T>, Vec<I>);

/// This trait contains functions to handle Simple RDF graphs, which are basically to get the neighbourhood of RDF nodes
///
///
pub trait SRDF: SRDFBasic {
    fn predicates_for_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashSet<Self::IRI>, Self::Err>;

    fn objects_for_subject_predicate(
        &self,
        subject: &Self::Subject,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err>;

    fn subjects_with_predicate_object(
        &self,
        pred: &Self::IRI,
        object: &Self::Term,
    ) -> Result<HashSet<Self::Subject>, Self::Err>;

    fn triples_with_predicate(&self, pred: &Self::IRI) -> Result<Vec<Triple<Self>>, Self::Err>;

    /*fn get_subjects_for_predicate_any_object(
        &self,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Subject>, Self::Err>;

    fn get_objects_for_predicate_any_subject(
        &self,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err>;*/

    /// Get the neighbours of a term
    /// This code creates an intermediate vector and is not very efficient
    /// TODO: return an iterator
    fn neighs(
        &self,
        node: &Self::Term,
    ) -> Result<ListOfIriAndTerms<Self::IRI, Self::Term>, Self::Err> {
        match Self::term_as_subject(node) {
            None => Ok(Vec::new()),
            Some(subject) => {
                let mut result = Vec::new();
                let preds = self.predicates_for_subject(&subject)?;
                for pred in preds {
                    let objs = self.objects_for_subject_predicate(&subject, &pred)?;
                    result.push((pred.clone(), objs));
                }
                Ok(result)
            }
        }
    }

    fn outgoing_arcs(
        &self,
        subject: &Self::Subject,
    ) -> Result<HasMapOfIriAndItem<Self::IRI, Self::Term>, Self::Err>;
    fn incoming_arcs(
        &self,
        object: &Self::Term,
    ) -> Result<HasMapOfIriAndItem<Self::IRI, Self::Subject>, Self::Err>;

    /// get outgoing arcs from a `node` taking into account only a controlled list of `preds`
    /// It returns a HashMap with the outgoing arcs and their values and a list of the predicates that have values and are not in the controlled list.
    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: Vec<Self::IRI>,
    ) -> Result<OutgoingArcs<Self::IRI, Self::Term>, Self::Err>;
}
