use std::collections::HashSet;
use std::hash::Hash;

use oxrdf::Triple as OxTriple;

use crate::model::rdf::MutableRdf;
use crate::model::rdf::Rdf;
use crate::model::rdf::RdfIri;
use crate::model::rdf::RdfSubject;
use crate::model::rdf::RdfTerm;
use crate::model::rdf::Triples;
use crate::model::Triple;

use super::error::GraphError;
use super::error::MutableGraphError;

pub type SimpleGraph = GenericSimpleGraph<OxTriple>;

pub struct GenericSimpleGraph<T: Triple>(HashSet<T>);

impl<T: Triple> Default for GenericSimpleGraph<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Triple> Rdf for GenericSimpleGraph<T> {
    type Triple = T;
    type Error = GraphError;

    fn triples_matching<'a>(
        &'a self,
        subject: Option<&'a RdfSubject<Self>>,
        predicate: Option<&'a RdfIri<Self>>,
        object: Option<&'a RdfTerm<Self>>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        let triples = self
            .0
            .iter()
            .filter(move |triple| match (subject, predicate, object) {
                (None, None, None) => true,
                (None, None, Some(obj)) => triple.obj() == obj,
                (None, Some(pred), None) => triple.pred() == pred,
                (None, Some(pred), Some(obj)) => triple.pred() == pred && triple.obj() == obj,
                (Some(subj), None, None) => triple.subj() == subj,
                (Some(subj), None, Some(obj)) => triple.subj() == subj && triple.obj() == obj,
                (Some(subj), Some(pred), None) => triple.subj() == subj && triple.pred() == pred,
                (Some(subj), Some(pred), Some(obj)) => {
                    triple.subj() == subj && triple.pred() == pred && triple.obj() == obj
                }
            });

        Ok(Box::new(triples))
    }
}

impl<T: Triple + Hash + Eq> MutableRdf for GenericSimpleGraph<T> {
    type MutableError = MutableGraphError;

    fn add_triple(
        &mut self,
        subject: RdfSubject<Self>,
        predicate: RdfIri<Self>,
        object: RdfTerm<Self>,
    ) -> Result<(), Self::MutableError> {
        self.0.insert(T::new(subject, predicate, object));
        Ok(())
    }

    fn remove_triple(&mut self, triple: &T) -> Result<(), Self::MutableError> {
        self.0.remove(triple);
        Ok(())
    }

    fn add_base(&mut self, base: &RdfIri<Self>) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_prefix(&mut self, alias: &str, iri: &RdfIri<Self>) -> Result<(), Self::Error> {
        todo!()
    }
}
