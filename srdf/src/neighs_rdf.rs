use std::collections::HashMap;
use std::collections::HashSet;

use crate::matcher::Any;
use crate::matcher::Matcher;
use crate::rdf_type;
use crate::Rdf;
use crate::Triple;

pub type IncomingArcs<R> = HashMap<<R as Rdf>::IRI, HashSet<<R as Rdf>::Subject>>;
pub type OutgoingArcs<R> = HashMap<<R as Rdf>::IRI, HashSet<<R as Rdf>::Term>>;
pub type OutgoingArcsFromList<R> = (OutgoingArcs<R>, Vec<<R as Rdf>::IRI>);

/// This trait contains functions to handle basic navigation in RDF graphs,
/// with methods that can get triples and the neighbourhood of RDF nodes
pub trait NeighsRDF: Rdf {
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

    fn incoming_arcs(&self, object: Self::Term) -> Result<IncomingArcs<Self>, Self::Err> {
        let mut results = IncomingArcs::<Self>::new();
        for triple in self.triples_with_object(object.clone())? {
            let (s, p, _) = triple.into_components();
            results.entry(p).or_default().insert(s);
        }
        Ok(results)
    }

    /// get all outgoing arcs from a subject
    fn outgoing_arcs(&self, subject: Self::Subject) -> Result<OutgoingArcs<Self>, Self::Err> {
        let mut results = OutgoingArcs::<Self>::new();
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
    ) -> Result<OutgoingArcsFromList<Self>, Self::Err> {
        let mut results = OutgoingArcs::<Self>::new();
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

    /*    fn get_subjects_for(
        &self,
        predicate: &Self::IRI,
        object: &Self::Term,
    ) -> Result<HashSet<Self::Term>, SRDFError> {
        let values = self
            .triples_matching(Any, predicate.clone(), object.clone())
            .map_err(|e| SRDFError::Srdf {
                error: e.to_string(),
            })?
            .map(Triple::into_subject)
            .map(Into::into)
            .collect();
        Ok(values)
    }

    fn get_path_for(
        &self,
        subject: &Self::Term,
        predicate: &Self::IRI,
    ) -> Result<Option<SHACLPath>, SRDFError> {
        match self.get_objects_for(subject, predicate)?
            .into_iter()
            .next()
        {
            Some(term) => {
                let obj: Object = Self::term_as_object(&term)?;
                match obj {
                    Object::Iri(iri_s) => Ok(Some(SHACLPath::iri(iri_s))),
                    Object::BlankNode(_) => todo!(),
                    Object::Literal(literal) => Err(SRDFError::SHACLUnexpectedLiteral {
                        lit: literal.to_string(),
                    }),
                }
            }
            None => Ok(None),
        }

    fn get_object_for(
            &self,
            subject: &Self::Term,
            predicate: &Self::IRI,
        ) -> Result<Option<RDFNode>, SRDFError> {
            match self.get_objects_for(subject, predicate)?
                .into_iter()
                .next()
            {
                Some(term) => {
                    let obj = Self::term_as_object(&term)?;
                    Ok(Some(obj))
                },
                None => Ok(None),
            }
        }

    fn get_objects_for(
            &self,
            subject: &Self::Term,
            predicate: &Self::IRI,
        ) -> Result<HashSet<Self::Term>, SRDFError> {
            let subject: Self::Subject = match Self::term_as_subject(subject) {
                Ok(subject) => subject,
                Err(_) => {
                    return Err(SRDFError::SRDFTermAsSubject {
                        subject: format!("{subject}"),
                    })
                }
            };

            let triples = store
                .triples_matching(subject, predicate.clone(), Any)
                .map_err(|e| SRDFError::Srdf {
                    error: e.to_string(),
                })?
                .map(Triple::into_object)
                .collect();

            Ok(triples)
        }
    }      */

    fn shacl_instances_of<O>(
        &self,
        cls: O,
    ) -> Result<impl Iterator<Item = Self::Subject>, Self::Err>
    where
        O: Matcher<Self::Term>,
    {
        let rdf_type: Self::IRI = rdf_type().clone().into();
        let subjects: HashSet<_> = self
            .triples_matching(Any, rdf_type, cls)?
            .map(Triple::into_subject)
            .collect();
        Ok(subjects.into_iter())
    }
}
