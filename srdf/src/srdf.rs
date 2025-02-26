use std::collections::{HashMap, HashSet};

use crate::{
    matcher::{Any, Matcher},
    Rdf, Triple,
};

pub type ListOfIriAndTerms<I, T> = Vec<(I, HashSet<T>)>;
pub type HashMapOfIriAndItem<I, T> = HashMap<I, HashSet<T>>;

type OutgoingArcs<I, T> = (HashMapOfIriAndItem<I, T>, Vec<I>);

/// This trait contains functions to handle Simple RDF graphs, which are basically to get the neighbourhood of RDF nodes
///
/// TODO: Consider alternative names: RDFGraphOps
pub trait Query: Rdf {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err>;

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
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        let triples = self.triples()?.filter_map(move |triple| {
            match subject == triple.subj() && predicate == triple.pred() && object == triple.obj() {
                true => Some(triple),
                false => None,
            }
        });
        Ok(triples)
    }

    fn triples_with_subject<S: Matcher<Self::Subject>>(
        &self,
        subject: S,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        self.triples_matching(subject, Any, Any)
    }

    fn triples_with_predicate<P: Matcher<Self::IRI>>(
        &self,
        predicate: P,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        self.triples_matching(Any, predicate, Any)
    }

    fn triples_with_object<O: Matcher<Self::Term>>(
        &self,
        object: O,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        self.triples_matching(Any, Any, object)
    }

    /// Get the neighbours of a term
    /// This code creates an intermediate vector and is not very efficient
    /// TODO: return an iterator
    fn neighs(
        &self,
        node: &Self::Term,
    ) -> Result<ListOfIriAndTerms<Self::IRI, Self::Term>, Self::Err> {
        let subject: Self::Subject = match node.clone().try_into() {
            Ok(subject) => subject,
            Err(_) => return Ok(Vec::default()), // TODO: this is inefficient
        };

        let preds = self
            .triples_with_subject(subject.clone())?
            .map(Triple::into_predicate);

        let mut result = Vec::new();
        for pred in preds {
            let objs = self
                .triples_matching(subject.clone(), pred.clone(), Any)?
                .map(Triple::into_object)
                .collect();
            result.push((pred, objs));
        }

        Ok(result)
    }

    fn incoming_arcs(
        &self,
        object: &Self::Term,
    ) -> Result<HashMapOfIriAndItem<Self::IRI, Self::Subject>, Self::Err> {
        let mut results: HashMapOfIriAndItem<Self::IRI, Self::Subject> = HashMap::new();
        for triple in self.triples_with_object(object.clone())? {
            let (s, p, _) = triple.into_components();
            results.entry(p).or_default().insert(s);
        }
        Ok(results)
    }

    fn outgoing_arcs(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashMapOfIriAndItem<Self::IRI, Self::Term>, Self::Err> {
        let mut results: HashMapOfIriAndItem<Self::IRI, Self::Term> = HashMap::new();
        for triple in self.triples_with_subject(subject.clone())? {
            let (_, p, o) = triple.into_components();
            results.entry(p).or_default().insert(o);
        }
        Ok(results)
    }

    /// get outgoing arcs from a `node` taking into account only a controlled list of `preds`
    /// It returns a HashMap with the outgoing arcs and their values and a list of the predicates that have values and are not in the controlled list.
    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: &[Self::IRI],
    ) -> Result<OutgoingArcs<Self::IRI, Self::Term>, Self::Err> {
        let mut results: HashMapOfIriAndItem<Self::IRI, Self::Term> = HashMap::new();
        let mut remainder = Vec::new();
        for triple in self.triples_with_subject(subject.clone())? {
            let (_, p, o) = triple.into_components();
            if preds.contains(&p) {
                results.entry(p).or_default().insert(o);
            } else {
                remainder.push(p)
            }
        }
        Ok((results, remainder))
    }
}
