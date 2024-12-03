use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

use iri_s::IriS;
use prefixmap::PrefixMap;

use super::FromComponents;
use super::TObjectRef;
use super::TPredicateRef;
use super::TSubjectRef;
use super::Triple;

pub type OutgoingArcs<'a, R> = HashMap<TPredicateRef<'a, R>, HashSet<TObjectRef<'a, R>>>;
pub type IncomingArcs<'a, R> = HashMap<TPredicateRef<'a, R>, HashSet<TSubjectRef<'a, R>>>;

pub trait Triples<T: Triple> {
    type TripleIter: Iterator<Item = T>;
    type Error: Error + 'static;

    fn triples(&self) -> Result<Self::TripleIter, Self::Error>;
}

/// This trait provides methods to handle Simple RDF graphs.
///
/// * Finding the triples provided a given pattern
/// * Obtaining the neighborhood of a node
pub trait Rdf<T: Triple>: Triples<T> {
    /// Obtain the `PrefixMap` associated with the graph, if any.
    fn prefixmap(&self) -> Option<PrefixMap>;

    /// An iterator over all the subjects in the graph.
    fn subjects(&self) -> Result<impl Iterator<Item = TSubjectRef<T>>, Self::Error> {
        Ok(self.triples()?.map(Triple::subject))
    }

    /// An iterator over all the predicates in the graph.
    fn predicates(&self) -> Result<impl Iterator<Item = TPredicateRef<T>>, Self::Error> {
        Ok(self.triples()?.map(Triple::predicate))
    }

    /// An iterator over all the objects in the graph.
    fn objects(&self) -> Result<impl Iterator<Item = TObjectRef<T>>, Self::Error> {
        Ok(self.triples()?.map(Triple::object))
    }

    /// An iterator over all the triples in the graph matching a basic graph pattern.
    fn triples_matching<'a>(
        &'a self,
        subject: Option<TSubjectRef<'a, T>>,
        predicate: Option<TPredicateRef<'a, T>>,
        object: Option<TObjectRef<'a, T>>,
    ) -> Result<impl Iterator<Item = T>, Self::Error> {
        let triples = self.triples()?.filter(move |triple| {
            let is_subject_match = subject.map_or(true, |subj| triple.subject() == subj);
            let is_predicate_match = predicate.map_or(true, |pred| triple.predicate() == pred);
            let is_object_match = object.map_or(true, |obj| triple.object() == obj);
            is_subject_match && is_predicate_match && is_object_match
        });
        Ok(triples)
    }

    /// An iterator over all the triples in the graph with a given subject.
    fn triples_with_subject<'a>(
        &'a self,
        subject: TSubjectRef<'a, T>,
    ) -> Result<impl Iterator<Item = T>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    /// An iterator over all the triples in the graph with a given predicate.
    fn triples_with_predicate<'a>(
        &'a self,
        predicate: TPredicateRef<'a, T>,
    ) -> Result<impl Iterator<Item = T>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    /// An iterator over all the triples in the graph with a given object.
    fn triples_with_object<'a>(
        &'a self,
        object: TObjectRef<'a, T>,
    ) -> Result<impl Iterator<Item = T>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    /// An iterator over all the objects in the graph that are neighbors of a given subject.
    fn neighs<'a>(
        &'a self,
        node: TSubjectRef<'a, T>,
    ) -> Result<impl Iterator<Item = TObjectRef<'a, T>>, Self::Error> {
        Ok(self
            .triples_with_subject(node)?
            .map(|triple| triple.object()))
    }

    /// An iterator over all the outgoing arcs from a given subject.
    fn outgoing_arcs<'a>(
        &'a self,
        subject: TSubjectRef<'a, T>,
    ) -> Result<OutgoingArcs<'a, T>, Self::Error> {
        let mut results: OutgoingArcs<T> = HashMap::new();
        for triple in self.triples_with_subject(subject)? {
            let (_, p, o) = triple.spo();
            results.entry(p).or_default().insert(o);
        }
        Ok(results)
    }

    /// An iterator over all the incoming arcs to a given object.
    fn incoming_arcs<'a>(
        &'a self,
        object: TObjectRef<'a, T>,
    ) -> Result<IncomingArcs<'a, T>, Self::Error> {
        let mut results: IncomingArcs<T> = HashMap::new();
        for triple in self.triples_with_object(object)? {
            let (s, p, _) = triple.spo();
            results.entry(p).or_default().insert(s);
        }
        Ok(results)
    }
}

/// Provides the functionality to implementors of being mutable.
pub trait MutableRdf {
    type Triple: FromComponents;
    type MutableRdfError;

    /// Add a triple to the graph.
    fn add_triple(&mut self, triple: Self::Triple) -> Result<(), Self::MutableRdfError>;

    /// Add a set of triples to the graph.
    fn add_triple_from_components(
        &mut self,
        subject: impl Into<<Self::Triple as FromComponents>::Subject>,
        predicate: impl Into<<Self::Triple as FromComponents>::Predicate>,
        object: impl Into<<Self::Triple as FromComponents>::Object>,
    ) -> Result<(), Self::MutableRdfError> {
        self.add_triple(Self::Triple::from_spo(subject, predicate, object))
    }

    /// Remove a triple from the graph.
    fn remove_triple(&mut self, triple: &Self::Triple) -> Result<(), Self::MutableRdfError>;

    /// Add a base to the graph.
    fn add_base(&mut self, base: IriS) -> Result<(), Self::MutableRdfError>;

    /// Add a prefix to the graph.
    fn add_prefix(&mut self, alias: &str, iri: IriS) -> Result<(), Self::MutableRdfError>;
}

/// Represents RDF graphs that contain a focus node.
///
/// This trait contains methods to get the focus node and for setting its value.
pub trait FocusRdf<'a>: Rdf {
    /// Set the value of the focus node
    fn set_focus(&mut self, focus: TObjectRef<'a, T>);

    /// Get the focus node if it exists
    fn get_focus(&self) -> Option<TObjectRef<T>>;
}
