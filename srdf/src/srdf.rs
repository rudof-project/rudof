use std::collections::{HashMap, HashSet};
//use std::hash::Hash;

use crate::{
    matcher::{Any, Matcher},
    Rdf, Triple as _,
};

pub type ListOfIriAndTerms<I, T> = Vec<(I, HashSet<T>)>;
pub type HasMapOfIriAndItem<I, T> = HashMap<I, HashSet<T>>;

type OutgoingArcs<I, T> = (HasMapOfIriAndItem<I, T>, Vec<I>);

/// This trait contains functions to handle Simple RDF graphs, which are basically to get the neighbourhood of RDF nodes
///
/// TODO: Consider alternative names: RDFGraphOps
pub trait Query: Rdf {
    fn triples(&self) -> impl Iterator<Item = Self::Triple>;

    /// Note to implementors: this function needs to retrieve all the triples of
    /// the graph. Therefore, for use-cases where the graph is large, this
    /// function should be implemented in a way that it does not retrieve all
    /// triples at once. As an example, for implementations of SPARQL, this
    /// function should be implemented to retrieve just the triples that match
    /// the given subject, predicate and object.
    fn triples_matching<S, P, O>(
        &self,
        subject: S,
        predicate: P,
        object: O,
    ) -> impl Iterator<Item = Self::Triple>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        self.triples().filter_map(move |triple| {
            match subject == triple.subj() && predicate == triple.pred() && object == triple.obj() {
                true => Some(triple),
                false => None,
            }
        })
    }

    fn triples_with_subject<S: Matcher<Self::Subject>>(
        &self,
        subject: S,
    ) -> impl Iterator<Item = Self::Triple> {
        self.triples_matching(subject, Any, Any)
    }

    fn triples_with_predicate<P: Matcher<Self::IRI>>(
        &self,
        predicate: P,
    ) -> impl Iterator<Item = Self::Triple> {
        self.triples_matching(Any, predicate, Any)
    }

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

    /// Get the neighbours of a term
    /// This code creates an intermediate vector and is not very efficient
    /// TODO: return an iterator
    fn neighs(
        &self,
        node: &Self::Term,
    ) -> Result<ListOfIriAndTerms<Self::IRI, Self::Term>, Self::Err> {
        match node.clone().try_into() {
            Ok(subject) => {
                let mut result = Vec::new();
                let preds = self.predicates_for_subject(&subject)?;
                for pred in preds {
                    let objs = self.objects_for_subject_predicate(&subject, &pred)?;
                    result.push((pred.clone(), objs));
                }
                Ok(result)
            }
            Err(_) => Ok(Vec::new()),
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
        preds: &[Self::IRI],
    ) -> Result<OutgoingArcs<Self::IRI, Self::Term>, Self::Err>;
}
