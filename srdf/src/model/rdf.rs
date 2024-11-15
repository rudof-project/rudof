use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;

use prefixmap::PrefixMap;

use super::Triple;

pub type Subject<R> = <<R as Rdf>::Triple as Triple>::Subject;
pub type Predicate<R> = <<R as Rdf>::Triple as Triple>::Iri;
pub type Object<R> = <<R as Rdf>::Triple as Triple>::Term;

pub type OutgoinArcs<R> = HashMap<Predicate<R>, HashSet<Object<R>>>;
pub type IncomingArcs<R> = HashMap<Predicate<R>, HashSet<Subject<R>>>;

/// This trait provides methods to handle Simple RDF graphs.
///
/// * Finding the triples provided a given pattern
/// * Obtaining the neighborhood of a node
pub trait Rdf {
    type Triple: Triple;
    type Error: Display + Debug;

    // Add explicit lifetime bounds on Iri and Term
    fn triples_matching<'a>(
        &self,
        subject: Option<&'a Subject<Self>>,
        predicate: Option<&'a Predicate<Self>>,
        object: Option<&'a Object<Self>>,
    ) -> Result<impl Iterator<Item = &Self::Triple>, Self::Error>;

    fn triples(&self) -> Result<impl Iterator<Item = &Self::Triple>, Self::Error> {
        self.triples_matching(None, None, None)
    }

    fn subjects(&self) -> Result<impl Iterator<Item = &Subject<Self>>, Self::Error> {
        let subjects = self.triples()?.map(Triple::subj);
        Ok(subjects)
    }

    fn predicates(&self) -> Result<impl Iterator<Item = &Predicate<Self>>, Self::Error> {
        let predicates = self.triples()?.map(Triple::pred);
        Ok(predicates)
    }

    fn objects(&self) -> Result<impl Iterator<Item = &Object<Self>>, Self::Error> {
        let objects = self.triples()?.map(Triple::obj);
        Ok(objects)
    }

    fn triples_with_subject<'a>(
        &'a self,
        subject: &'a Subject<Self>,
    ) -> Result<impl Iterator<Item = &Self::Triple>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    fn triples_with_predicate<'a>(
        &'a self,
        predicate: &'a Predicate<Self>,
    ) -> Result<impl Iterator<Item = &Self::Triple>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    fn triples_with_object<'a>(
        &'a self,
        object: &'a Object<Self>,
    ) -> Result<impl Iterator<Item = &Self::Triple>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    fn neighs<'a>(
        &'a self,
        node: &'a Subject<Self>,
    ) -> Result<impl Iterator<Item = &Object<Self>>, Self::Error> {
        let objects = self.triples_with_subject(node)?.map(Triple::obj);
        Ok(objects)
    }

    fn outgoing_arcs(&self, subject: &Subject<Self>) -> Result<OutgoinArcs<Self>, Self::Error> {
        let mut results: HashMap<Predicate<Self>, HashSet<Object<Self>>> = HashMap::new();
        for triple in self.triples_with_subject(subject)? {
            let pred = triple.pred().clone();
            let term = triple.obj().clone();
            match results.entry(pred) {
                Entry::Occupied(mut vs) => {
                    vs.get_mut().insert(term.clone());
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(HashSet::from([term.clone()]));
                }
            }
        }
        Ok(results)
    }

    fn incoming_arcs(&self, object: &Object<Self>) -> Result<IncomingArcs<Self>, Self::Error> {
        let mut results: HashMap<Predicate<Self>, HashSet<Subject<Self>>> = HashMap::new();
        for triple in self.triples_with_object(object)? {
            let pred = triple.pred().clone();
            let term = triple.subj().clone();
            match results.entry(pred) {
                Entry::Occupied(mut vs) => {
                    vs.get_mut().insert(term.clone());
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(HashSet::from([term.clone()]));
                }
            }
        }
        Ok(results)
    }

    fn prefixmap(&self) -> Option<PrefixMap>;
}
