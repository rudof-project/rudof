use std::collections::HashMap;
use std::collections::HashSet;

use prefixmap::PrefixMap;

use super::Term;
use super::Triple;

pub type Subject<R> = <<R as Rdf>::Triple as Triple>::Subject;
pub type Predicate<R> = <<R as Rdf>::Triple as Triple>::Iri;
pub type Object<R> = <<R as Rdf>::Triple as Triple>::Term;

pub type Literal<T> = <<T as Triple>::Term as Term>::Literal;
pub type Iri<T> = <<T as Triple>::Term as Term>::Iri;

pub type OutgoingArcs<R> = HashMap<Predicate<R>, HashSet<Object<R>>>;
pub type IncomingArcs<R> = HashMap<Predicate<R>, HashSet<Subject<R>>>;

/// This trait provides methods to handle Simple RDF graphs.
///
/// * Finding the triples provided a given pattern
/// * Obtaining the neighborhood of a node
pub trait Rdf {
    type Triple: Triple;
    type Error: std::error::Error + 'static;

    /// An iterator over all the triples matching a given pattern.
    fn triples_matching<'a>(
        &self,
        subject: Option<&'a Subject<Self>>,
        predicate: Option<&'a Predicate<Self>>,
        object: Option<&'a Object<Self>>,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Error>;

    /// An iterator over all the triples.
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Error> {
        self.triples_matching(None, None, None)
    }

    /// An iterator over all the subjects of the triples.
    fn subjects(&self) -> Result<impl Iterator<Item = Subject<Self>>, Self::Error> {
        let subjects = self.triples()?.map(Triple::into_subject);
        Ok(subjects)
    }

    /// An iterator over all the predicates of the triples.
    fn predicates(&self) -> Result<impl Iterator<Item = Predicate<Self>>, Self::Error> {
        let predicates = self.triples()?.map(Triple::into_predicate);
        Ok(predicates)
    }

    /// An iterator over all the objects of the triples.
    fn objects(&self) -> Result<impl Iterator<Item = Object<Self>>, Self::Error> {
        let objects = self.triples()?.map(Triple::into_object);
        Ok(objects)
    }

    /// An iterator over all the triples with a given subject.
    fn triples_with_subject<'a>(
        &'a self,
        subject: &'a Subject<Self>,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    /// An iterator over all the triples with a given predicate.
    fn triples_with_predicate<'a>(
        &'a self,
        predicate: &'a Predicate<Self>,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    /// An iterator over all the triples with a given object.
    fn triples_with_object<'a>(
        &'a self,
        object: &'a Object<Self>,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    /// An iterator over all the neighbours of a given subject.
    fn neighs<'a>(
        &'a self,
        node: &'a Subject<Self>,
    ) -> Result<impl Iterator<Item = Object<Self>>, Self::Error> {
        let objects = self.triples_with_subject(node)?.map(Triple::into_object);
        Ok(objects)
    }

    /// An iterator over all the outgoing arcs from a given subject.
    fn outgoing_arcs(&self, subject: &Subject<Self>) -> Result<OutgoingArcs<Self>, Self::Error> {
        let mut results: OutgoingArcs<Self> = HashMap::new();
        for triple in self.triples_with_subject(subject)? {
            let (_, p, o) = triple.spo();
            results.entry(p.clone()).or_default().insert(o.clone());
        }
        Ok(results)
    }

    /// An iterator over all the incoming arcs to a given object.
    fn incoming_arcs(&self, object: &Object<Self>) -> Result<IncomingArcs<Self>, Self::Error> {
        let mut results: IncomingArcs<Self> = HashMap::new();
        for triple in self.triples_with_object(object)? {
            let (s, p, _) = triple.spo();
            results.entry(p.clone()).or_default().insert(s.clone());
        }
        Ok(results)
    }
}

/// Provides the functionality to implementors of being mutable.
pub trait MutableRdf: Rdf {
    type MutableRdfError: std::error::Error + 'static;

    /// Add a triple to the graph.
    fn add_triple(&mut self, triple: Self::Triple) -> Result<(), Self::MutableRdfError>;

    /// Add a set of triples to the graph.
    fn add_triple_from_components(
        &mut self,
        subject: impl Into<Subject<Self>>,
        predicate: impl Into<Predicate<Self>>,
        object: impl Into<Object<Self>>,
    ) -> Result<(), Self::MutableRdfError> {
        self.add_triple(Self::Triple::from_spo(subject, predicate, object))
    }

    /// Remove a triple from the graph.
    fn remove_triple(&mut self, triple: &Self::Triple) -> Result<(), Self::MutableRdfError>;

    /// Add a base to the graph.
    fn add_base(&mut self, base: Predicate<Self>) -> Result<(), Self::MutableRdfError>;

    /// Add a prefix to the graph.
    fn add_prefix(
        &mut self,
        alias: &str,
        iri: Predicate<Self>,
    ) -> Result<(), Self::MutableRdfError>;
}

/// Represents RDF graphs that contain a focus node.
///
/// The trait contains methods to get the focus node and for setting its value.
pub trait FocusRdf: Rdf {
    /// Set the value of the focus node.
    fn set_focus(&mut self, focus: Object<Self>);

    /// Get the focus node if it exists.
    fn get_focus(&self) -> Option<&Object<Self>>;
}

pub trait PrefixMapRdf: Rdf {
    /// Get the prefixmap.
    fn prefixmap(&self) -> &PrefixMap;
}
