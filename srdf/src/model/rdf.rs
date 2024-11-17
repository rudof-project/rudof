use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use prefixmap::PrefixMap;

use super::Term;
use super::Triple;

pub type Triples<'a, R> = Box<dyn Iterator<Item = <R as Rdf>::Triple<'a>> + 'a>;
pub type Subjects<'a, R> = Box<dyn Iterator<Item = TSubject<'a, R>> + 'a>;
pub type Predicates<'a, R> = Box<dyn Iterator<Item = TPredicate<'a, R>> + 'a>;
pub type Objects<'a, R> = Box<dyn Iterator<Item = TObject<'a, R>> + 'a>;

pub type TSubject<'a, R> = <<R as Rdf>::Triple<'a> as Triple>::Subject<'a>;
pub type TPredicate<'a, R> = <<R as Rdf>::Triple<'a> as Triple>::Iri<'a>;
pub type TObject<'a, R> = <<R as Rdf>::Triple<'a> as Triple>::Term<'a>;

pub type TLiteral<'a, T> = <<T as Triple>::Term<'a> as Term>::Literal<'a>;
pub type TIri<'a, T> = <<T as Triple>::Term<'a> as Term>::Iri<'a>;

pub type OutgoingArcs<'a, R> = HashMap<TPredicate<'a, R>, HashSet<TObject<'a, R>>>;
pub type IncomingArcs<'a, R> = HashMap<TPredicate<'a, R>, HashSet<TSubject<'a, R>>>;

/// This trait provides methods to handle Simple RDF graphs.
///
/// * Finding the triples provided a given pattern
/// * Obtaining the neighborhood of a node
pub trait Rdf {
    type Triple<'a>: Triple + Clone + Debug + Eq + Hash
    where
        Self: 'a;

    type Error: Display + Debug;

    fn prefixmap(&self) -> Option<PrefixMap>;

    fn triples_matching(
        &self,
        subject: Option<TSubject<Self>>,
        predicate: Option<TPredicate<Self>>,
        object: Option<TObject<Self>>,
    ) -> Result<Triples<Self>, Self::Error>;

    fn triples(&self) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, None, None)
    }

    fn subjects(&self) -> Result<Subjects<Self>, Self::Error> {
        let subjects = self.triples()?.map(Triple::subj);
        Ok(subjects)
    }

    fn predicates(&self) -> Result<Predicates<Self>, Self::Error> {
        let predicates = self.triples()?.map(Triple::pred);
        Ok(predicates)
    }

    fn objects(&self) -> Result<Objects<Self>, Self::Error> {
        let objects = self.triples()?.map(Triple::obj);
        Ok(objects)
    }

    fn triples_with_subject(&self, subject: TSubject<Self>) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    fn triples_with_predicate(
        &self,
        predicate: TPredicate<Self>,
    ) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    fn triples_with_object(&self, object: TObject<Self>) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    fn neighs(&self, node: TSubject<Self>) -> Result<Objects<Self>, Self::Error> {
        let objects = self.triples_with_subject(node)?.map(Triple::obj);
        Ok(objects)
    }

    fn outgoing_arcs(&self, subject: &TSubject<Self>) -> Result<OutgoingArcs<Self>, Self::Error> {
        let mut results: HashMap<TPredicate<Self>, HashSet<TObject<Self>>> = HashMap::new();
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

    fn incoming_arcs(&self, object: &TObject<Self>) -> Result<IncomingArcs<Self>, Self::Error> {
        let mut results: HashMap<TPredicate<Self>, HashSet<TSubject<Self>>> = HashMap::new();
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
}
