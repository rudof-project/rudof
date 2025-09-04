use std::collections::HashMap;
use std::collections::HashSet;

use crate::Object;
use crate::RDFError;
use crate::Rdf;
use crate::SHACLPath;
use crate::Triple;
use crate::matcher::Any;
use crate::matcher::Matcher;
use crate::rdf_type;

pub type IncomingArcs<R> = HashMap<<R as Rdf>::IRI, HashSet<<R as Rdf>::Subject>>;
pub type OutgoingArcs<R> = HashMap<<R as Rdf>::IRI, HashSet<<R as Rdf>::Term>>;
pub type OutgoingArcsFromList<R> = (OutgoingArcs<R>, Vec<<R as Rdf>::IRI>);

/// This trait contains functions to handle basic navigation in RDF graphs,
/// with methods that can get triples and the neighbourhood of RDF nodes
pub trait NeighsRDF: Rdf {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err>;

    fn contains<S, P, O>(&self, subject: S, predicate: P, object: O) -> Result<bool, Self::Err>
    where
        S: Matcher<Self::Subject> + Clone,
        P: Matcher<Self::IRI> + Clone,
        O: Matcher<Self::Term> + Clone,
    {
        let mut iter = self.triples_matching(subject, predicate, object)?;
        Ok(iter.next().is_some())
    }

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
        S: Matcher<Self::Subject> + Clone,
        P: Matcher<Self::IRI> + Clone,
        O: Matcher<Self::Term> + Clone;

    fn triples_with_subject(
        &self,
        subject: Self::Subject,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        self.triples_matching(subject, Any, Any)
    }

    /// We define this function to get all triples with a specific subject and predicate
    /// This function could be optimized by some implementations
    fn triples_with_subject_predicate(
        &self,
        subject: Self::Subject,
        predicate: Self::IRI,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        self.triples_matching(subject, predicate, Any)
    }

    fn triples_with_predicate(
        &self,
        predicate: Self::IRI,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        self.triples_matching(Any, predicate, Any)
    }

    fn triples_with_object(
        &self,
        object: Self::Term,
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
        tracing::debug!("Getting outgoing arcs for subject: {}", subject);
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

    fn shacl_instances_of<O>(
        &self,
        cls: O,
    ) -> Result<impl Iterator<Item = Self::Subject>, Self::Err>
    where
        O: Matcher<Self::Term> + Clone,
    {
        let rdf_type: Self::IRI = rdf_type().clone().into();
        let subjects: HashSet<_> = self
            .triples_matching(Any, rdf_type, cls)?
            .map(Triple::into_subject)
            .collect();
        Ok(subjects.into_iter())
    }

    fn object_for(
        &self,
        subject: &Self::Term,
        predicate: &Self::IRI,
    ) -> Result<Option<Object>, RDFError> {
        match self.objects_for(subject, predicate)?.into_iter().next() {
            Some(term) => {
                let obj = Self::term_as_object(&term)?;
                Ok(Some(obj))
            }
            None => Ok(None),
        }
    }

    fn objects_for_shacl_path(
        &self,
        subject: &Self::Term,
        path: &SHACLPath,
    ) -> Result<HashSet<Self::Term>, RDFError> {
        match path {
            SHACLPath::Predicate { pred } => {
                let pred: Self::IRI = pred.clone().into();
                self.objects_for(subject, &pred)
            }
            SHACLPath::Alternative { paths } => {
                let mut all_objects = HashSet::new();
                for path in paths {
                    let objects = self.objects_for_shacl_path(subject, path)?;
                    all_objects.extend(objects);
                }
                Ok(all_objects)
            }
            SHACLPath::Sequence { paths } => match paths.as_slice() {
                [] => Ok(HashSet::from([subject.clone()])),
                [first, rest @ ..] => {
                    let first_objects = self.objects_for_shacl_path(subject, first)?;
                    let mut all_objects = HashSet::new();
                    for obj in first_objects {
                        let intermediate_objects = self.objects_for_shacl_path(
                            &obj,
                            &SHACLPath::Sequence {
                                paths: rest.to_vec(),
                            },
                        )?;
                        all_objects.extend(intermediate_objects);
                    }
                    Ok(all_objects)
                }
            },
            SHACLPath::Inverse { path } => {
                let objects = self.subjects_for(&path.pred().unwrap().clone().into(), subject)?;
                Ok(objects)
            }
            SHACLPath::ZeroOrMore { path } => {
                let mut all_objects = HashSet::new();
                all_objects.insert(subject.clone());

                let mut to_process = vec![subject.clone()];
                while let Some(current) = to_process.pop() {
                    let next_objects = self.objects_for_shacl_path(&current, path)?;
                    for obj in next_objects {
                        if all_objects.insert(obj.clone()) {
                            to_process.push(obj);
                        }
                    }
                }
                Ok(all_objects)
            }
            SHACLPath::OneOrMore { path } => {
                let mut all_objects = HashSet::new();
                let first_objects = self.objects_for_shacl_path(subject, path)?;
                all_objects.extend(first_objects.clone());

                let mut to_process: Vec<Self::Term> = first_objects.into_iter().collect();
                while let Some(current) = to_process.pop() {
                    let next_objects = self.objects_for_shacl_path(&current, path)?;
                    for obj in next_objects {
                        if all_objects.insert(obj.clone()) {
                            to_process.push(obj);
                        }
                    }
                }
                Ok(all_objects)
            }
            SHACLPath::ZeroOrOne { path } => {
                let mut all_objects = HashSet::new();
                all_objects.insert(subject.clone());
                let next_objects = self.objects_for_shacl_path(subject, path)?;
                all_objects.extend(next_objects);
                Ok(all_objects)
            }
        }
    }

    fn objects_for(
        &self,
        subject: &Self::Term,
        predicate: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, RDFError> {
        let subject: Self::Subject = Self::term_as_subject(subject)?;
        let subject_str = format!("{subject}");
        let predicate_str = format!("{predicate}");
        let triples = self
            .triples_matching(subject, predicate.clone(), Any)
            .map_err(|e| RDFError::ErrorObjectsFor {
                subject: subject_str,
                predicate: predicate_str,
                error: e.to_string(),
            })?
            .map(Triple::into_object)
            .collect();

        Ok(triples)
    }

    fn subjects_for(
        &self,
        predicate: &Self::IRI,
        object: &Self::Term,
    ) -> Result<HashSet<Self::Term>, RDFError> {
        let values = self
            .triples_matching(Any, predicate.clone(), object.clone())
            .map_err(|e| RDFError::ErrorSubjectsFor {
                predicate: format!("{predicate}"),
                object: format!("{object}"),
                error: e.to_string(),
            })?
            .map(Triple::into_subject)
            .map(Into::into)
            .collect();
        Ok(values)
    }
}
