use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;

use prefixmap::PrefixMap;

use super::TObjectRef;
use super::TPredicateRef;
use super::TSubjectRef;
use super::Triple;

type Subject<'a, R> = <<R as Rdf>::Triple<'a> as Triple>::Subject;
type Predicate<'a, R> = <<R as Rdf>::Triple<'a> as Triple>::Iri;
type Object<'a, R> = <<R as Rdf>::Triple<'a> as Triple>::Term;

pub type Triples<'a, R> = Box<dyn Iterator<Item = <R as Rdf>::Triple<'a>> + 'a>;
pub type Subjects<'a, R> = Box<dyn Iterator<Item = Subject<'a, R>> + 'a>;
pub type Predicates<'a, R> = Box<dyn Iterator<Item = Predicate<'a, R>> + 'a>;
pub type Objects<'a, R> = Box<dyn Iterator<Item = Object<'a, R>> + 'a>;

pub type OutgoingArcs<'a, R> = HashMap<Predicate<'a, R>, HashSet<Object<'a, R>>>;
pub type IncomingArcs<'a, R> = HashMap<Predicate<'a, R>, HashSet<Subject<'a, R>>>;

/// This trait provides methods to handle Simple RDF graphs.
///
/// * Finding the triples provided a given pattern
/// * Obtaining the neighborhood of a node
pub trait Rdf {
    type Triple<'a>: Triple + Clone + Debug + Eq + Hash
    where
        Self: 'a;

    type Error: Error + 'static;

    /// Obtain the `PrefixMap` associated with the graph, if any.
    fn prefixmap(&self) -> Option<PrefixMap>;

    /// An iterator over all the triples in the graph.
    fn triples(&self) -> Result<Triples<Self>, Self::Error>;

    /// An iterator over all the triples in the graph matching a basic graph pattern.
    fn triples_matching(
        &self,
        subject: Option<TSubjectRef<Self::Triple<'_>>>,
        predicate: Option<TPredicateRef<Self::Triple<'_>>>,
        object: Option<TObjectRef<Self::Triple<'_>>>,
    ) -> Result<Triples<Self>, Self::Error> {
        let triples = self.triples()?.filter(|triple| {
            subject.map_or(true, |subj| triple.subject() == subj)
                && predicate.map_or(true, |pred| triple.predicate() == pred)
                && object.map_or(true, |obj| triple.object() == obj)
        });
        Ok(Box::new(triples))
    }

    /// An iterator over all the subjects in the graph, it may include duplicates.
    fn subjects(&self) -> Result<Subjects<Self>, Self::Error> {
        let subjects = self.triples()?.map(Triple::as_subject);
        Ok(Box::new(subjects))
    }

    /// An iterator over all the predicates in the graph, it may include duplicates.
    fn predicates(&self) -> Result<Predicates<Self>, Self::Error> {
        let predicates = self.triples()?.map(Triple::as_predicate);
        Ok(Box::new(predicates))
    }

    /// An iterator over all the subjects in the graph, it may include duplicates.
    fn objects(&self) -> Result<Objects<Self>, Self::Error> {
        let objects = self.triples()?.map(Triple::as_object);
        Ok(Box::new(objects))
    }

    fn triples_with_subject(
        &self,
        subject: TSubjectRef<Self::Triple<'_>>,
    ) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    fn triples_with_predicate(
        &self,
        predicate: TPredicateRef<Self::Triple<'_>>,
    ) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    fn triples_with_object(
        &self,
        object: TObjectRef<Self::Triple<'_>>,
    ) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    fn neighs(&self, node: TSubjectRef<Self::Triple<'_>>) -> Result<Objects<Self>, Self::Error> {
        let objects = self.triples_with_subject(node)?.map(Triple::as_object);
        Ok(Box::new(objects))
    }

    fn outgoing_arcs(
        &self,
        subject: TSubjectRef<Self::Triple<'_>>,
    ) -> Result<OutgoingArcs<Self>, Self::Error> {
        let mut results: OutgoingArcs<Self> = HashMap::new();
        for triple in self.triples_with_subject(subject)? {
            let (_, p, o) = triple.as_spo();
            match results.entry(p) {
                Entry::Occupied(mut vs) => {
                    vs.get_mut().insert(o);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(HashSet::from([o]));
                }
            }
        }
        Ok(results)
    }

    fn incoming_arcs(
        &self,
        object: TObjectRef<Self::Triple<'_>>,
    ) -> Result<IncomingArcs<Self>, Self::Error> {
        let mut results: IncomingArcs<Self> = HashMap::new();
        for triple in self.triples_with_object(object)? {
            let (s, p, _) = triple.as_spo();
            match results.entry(p) {
                Entry::Occupied(mut vs) => {
                    vs.get_mut().insert(s);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(HashSet::from([s]));
                }
            }
        }
        Ok(results)
    }
}
