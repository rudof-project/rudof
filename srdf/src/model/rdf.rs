use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

use iri_s::IriS;
use prefixmap::PrefixMap;

use crate::model::Iri;
use crate::model::Subject;
use crate::model::Term;

use super::TObjectRef;
use super::TPredicateRef;
use super::TSubjectRef;
use super::Triple;

pub type Triples<'a, R> = Box<dyn Iterator<Item = <R as Rdf>::Triple<'a>> + 'a>;
pub type Subjects<'a, R> = Box<dyn Iterator<Item = TSubjectRef<'a, R>> + 'a>;
pub type Predicates<'a, R> = Box<dyn Iterator<Item = TPredicateRef<'a, R>> + 'a>;
pub type Objects<'a, R> = Box<dyn Iterator<Item = TObjectRef<'a, R>> + 'a>;

pub type OutgoingArcs<'a, R> = HashMap<TPredicateRef<'a, R>, HashSet<TObjectRef<'a, R>>>;
pub type IncomingArcs<'a, R> = HashMap<TPredicateRef<'a, R>, HashSet<TSubjectRef<'a, R>>>;

/// This trait provides methods to handle Simple RDF graphs.
///
/// * Finding the triples provided a given pattern
/// * Obtaining the neighborhood of a node
pub trait Rdf {
    type Triple<'a>: Triple
    where
        Self: 'a;

    type Error: Error + 'static;

    /// Obtain the `PrefixMap` associated with the graph, if any.
    fn prefixmap(&self) -> Option<PrefixMap>;

    /// An iterator over all the triples in the graph.
    fn triples(&self) -> Result<Triples<Self>, Self::Error>;

    /// An iterator over all the triples in the graph matching a basic graph pattern.
    fn triples_matching<'a>(
        &'a self,
        subject: Option<TSubjectRef<'a, Self::Triple<'a>>>,
        predicate: Option<TPredicateRef<'a, Self::Triple<'a>>>,
        object: Option<TObjectRef<'a, Self::Triple<'a>>>,
    ) -> Result<Triples<Self>, Self::Error> {
        let triples = self.triples()?.filter(move |triple| {
            let is_subject_match = subject.map_or(true, |subj| triple.subject().eq(&subj));
            let is_predicate_match = predicate.map_or(true, |pred| triple.predicate().eq(&pred));
            let is_object_match = object.map_or(true, |obj| triple.object().eq(&obj));
            is_subject_match && is_predicate_match && is_object_match
        });
        Ok(Box::new(triples))
    }

    /// An iterator that consumes all the subjects in the graph, it may include duplicates.
    fn subjects(
        &self,
    ) -> Result<impl Iterator<Item = <Self::Triple<'_> as Triple>::Subject>, Self::Error> {
        let subjects = self.triples()?.map(Triple::as_subject);
        Ok(subjects)
    }

    /// An iterator that consumes all the predicates in the graph, it may include duplicates.
    fn predicates(
        &self,
    ) -> Result<impl Iterator<Item = <Self::Triple<'_> as Triple>::Iri>, Self::Error> {
        let predicates = self.triples()?.map(Triple::as_predicate);
        Ok(predicates)
    }

    /// An iterator that consumes all the objects in the graph, it may include duplicates.
    fn objects(
        &self,
    ) -> Result<impl Iterator<Item = <Self::Triple<'_> as Triple>::Term>, Self::Error> {
        let objects = self.triples()?.map(Triple::as_object);
        Ok(objects)
    }

    fn triples_with_subject<'a>(
        &'a self,
        subject: TSubjectRef<'a, Self::Triple<'a>>,
    ) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    fn triples_with_predicate<'a>(
        &'a self,
        predicate: TPredicateRef<'a, Self::Triple<'a>>,
    ) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    fn triples_with_object<'a>(
        &'a self,
        object: TObjectRef<'a, Self::Triple<'a>>,
    ) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    fn neighs<'a>(
        &'a self,
        node: TSubjectRef<'a, Self::Triple<'a>>,
    ) -> Result<Objects<'a, Self::Triple<'a>>, Self::Error> {
        let objects = self.triples_with_subject(node)?;
        Ok(Box::new(objects))
    }

    fn outgoing_arcs(
        &self,
        subject: TSubjectRef<Self::Triple<'_>>,
    ) -> Result<OutgoingArcs<Self::Triple<'_>>, Self::Error> {
        let mut results: OutgoingArcs<Self::Triple<'_>> = HashMap::new();
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
    ) -> Result<IncomingArcs<Self::Triple<'_>>, Self::Error> {
        let mut results: IncomingArcs<Self::Triple<'_>> = HashMap::new();
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

/// Provides the functionality to implementors of being mutable.
pub trait MutableRdf: Rdf {
    type MutableRdfError;

    fn add_triple(
        &mut self,
        subject: <Self::Triple<'_> as Triple>::Subject,
        predicate: <Self::Triple<'_> as Triple>::Iri,
        object: <Self::Triple<'_> as Triple>::Term,
    ) -> Result<(), Self::MutableRdfError>;

    fn remove_triple(&mut self, triple: &Self::Triple<'_>) -> Result<(), Self::MutableRdfError>;

    fn add_base(&mut self, base: IriS) -> Result<(), Self::MutableRdfError>;
    fn add_prefix(&mut self, alias: &str, iri: IriS) -> Result<(), Self::MutableRdfError>;
}

/// Represents RDF graphs that contain a focus node.
///
/// This trait contains methods to get the focus node and for setting its value.
pub trait FocusRdf: Rdf {
    /// Set the value of the focus node
    fn set_focus<'a>(&mut self, focus: TObjectRef<'a, Self::Triple<'a>>);

    /// Get the focus node if it exists
    fn get_focus(&self) -> Option<TObjectRef<Self::Triple<'_>>>;
}
