use crate::rdf_core::vocabs::RdfVocab;
use crate::rdf_core::{
    Any, Matcher, RDFError, Rdf, SHACLPath,
    term::{Object, Triple},
};
use std::collections::{HashMap, HashSet};
//----------------------------------------------------------------
// Type aliases for common RDF navigation patterns
//----------------------------------------------------------------

/// Maps predicates to sets of subjects (inverse navigation)
pub type IncomingArcs<R> = HashMap<<R as Rdf>::IRI, HashSet<<R as Rdf>::Subject>>;
/// Maps predicates to sets of objects (forward navigation)
pub type OutgoingArcs<R> = HashMap<<R as Rdf>::IRI, HashSet<<R as Rdf>::Term>>;
/// Filtered outgoing arcs with reminder predicates
pub type OutgoingArcsFromList<R> = (OutgoingArcs<R>, Vec<<R as Rdf>::IRI>);

/// Trait for navigating RDF graphs and querying triples.
///
/// This trait extends [`Rdf`] with methods for retrieving triples based on
/// subject-predicate-object patterns, exploring node neighborhoods, and
/// following SHACL property paths. All query methods support flexible
/// matching using the [`Matcher`] trait, allowing exact matches or wildcards.
///
/// # Graph Navigation
///
/// The trait provides two primary navigation models:
///
/// - **Triple queries**: Retrieve triples matching specific patterns
/// - **Arc-based navigation**: Explore incoming and outgoing relationships
pub trait NeighsRDF: Rdf {
    /// Returns an iterator over all triples in the RDF graph.
    ///
    /// This method provides access to the complete set of triples. For large
    /// graphs, implementations should return a lazy iterator that retrieves
    /// triples incrementally rather than loading everything into memory.
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err>;

    /// Checks whether the graph contains at least one triple matching the pattern.
    ///
    /// # Arguments
    ///
    /// * `subject` - Matcher for the subject (use [`Any`] for wildcard)
    /// * `predicate` - Matcher for the predicate (use [`Any`] for wildcard)
    /// * `object` - Matcher for the object (use [`Any`] for wildcard)
    fn contains<S, P, O>(&self, subject: &S, predicate: &P, object: &O) -> Result<bool, Self::Err>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        let mut iter = self.triples_matching(subject, predicate, object)?;
        Ok(iter.next().is_some())
    }

    /// Returns an iterator over triples matching the given pattern.
    ///
    /// This is the core query method that all other triple queries delegate to.
    /// Each parameter accepts a [`Matcher`], allowing exact values or wildcards
    /// via [`Any`].
    ///
    /// # Implementation Note
    ///
    /// This function must retrieve triples from the graph, but should **not**
    /// load all triples into memory for large graphs. For SPARQL-based
    /// implementations, translate the pattern into a SPARQL query that
    /// retrieves only matching triples incrementally.
    ///
    /// # Arguments
    ///
    /// * `subject` - Matcher for the subject position
    /// * `predicate` - Matcher for the predicate position
    /// * `object` - Matcher for the object position
    fn triples_matching<S, P, O>(
        &self,
        subject: &S,
        predicate: &P,
        object: &O,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>;

    /// Returns all triples with the specified subject.
    ///
    /// Equivalent to `triples_matching(subject, Any, Any)`.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject to match
    fn triples_with_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err> {
        self.triples_matching(subject, &Any, &Any)
    }

    /// Returns all triples with the specified subject and predicate.
    ///
    /// Equivalent to `triples_matching(subject, predicate, Any)`.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject to match
    /// * `predicate` - The predicate to match
    fn triples_with_subject_predicate(
        &self,
        subject: &Self::Subject,
        predicate: &Self::IRI,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err> {
        self.triples_matching(subject, predicate, &Any)
    }

    /// Returns all triples with the specified predicate.
    ///
    /// Equivalent to `triples_matching(Any, predicate, Any)`.
    ///
    /// # Arguments
    ///
    /// * `predicate` - The predicate to match
    fn triples_with_predicate(
        &self,
        predicate: &Self::IRI,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err> {
        self.triples_matching(&Any, predicate, &Any)
    }

    /// Returns all triples with the specified predicate and object.
    ///
    /// Equivalent to `triples_matching(Any, predicate, object)`.
    ///
    /// # Arguments
    ///
    /// * `predicate` - The predicate to match
    /// * `object` - The object to match
    fn triples_with_predicate_object(
        &self,
        predicate: &Self::IRI,
        object: &Self::Term,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err> {
        self.triples_matching(&Any, predicate, object)
    }

    /// Returns all triples with the specified object.
    ///
    /// Equivalent to `triples_matching(Any, Any, object)`.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to match
    fn triples_with_object(&self, object: &Self::Term) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err> {
        self.triples_matching(&Any, &Any, object)
    }

    /// Returns all incoming arcs (predicates and subjects) pointing to an object.
    ///
    /// This method performs reverse navigation, finding all subjects that have
    /// relationships pointing to the specified object, grouped by predicate.
    ///
    /// # Arguments
    ///
    /// * `object` - The object term to find incoming relationships for
    fn incoming_arcs(&self, object: &Self::Term) -> Result<IncomingArcs<Self>, Self::Err> {
        let mut results = IncomingArcs::<Self>::new();
        for triple in self.triples_with_object(object)? {
            let (s, p, _) = triple.into_components();
            results.entry(p).or_default().insert(s);
        }
        Ok(results)
    }

    /// Returns all outgoing arcs (predicates and objects) from a subject.
    ///
    /// This method performs forward navigation, finding all predicates and
    /// their associated objects for the specified subject.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject to find outgoing relationships for
    fn outgoing_arcs(&self, subject: &Self::Subject) -> Result<OutgoingArcs<Self>, Self::Err> {
        let mut results = OutgoingArcs::<Self>::new();
        for triple in self.triples_with_subject(subject)? {
            let (_, p, o) = triple.into_components();
            results.entry(p).or_default().insert(o);
        }
        Ok(results)
    }

    /// Returns filtered outgoing arcs and remainder predicates.
    ///
    /// This method retrieves outgoing arcs from a subject, but only includes
    /// predicates that appear in the provided allowlist. Predicates not in
    /// the list are collected separately in the remainder vector.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject to query
    /// * `preds` - A slice of predicates to include in the filtered results
    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: &[Self::IRI],
    ) -> Result<OutgoingArcsFromList<Self>, Self::Err> {
        let mut results = OutgoingArcs::<Self>::new();
        let mut reminder = Vec::new();

        for triple in self.triples_with_subject(subject)? {
            let (_, p, o) = triple.into_components();

            if preds.contains(&p) {
                results.entry(p).or_default().insert(o);
            } else {
                reminder.push(p);
            }
        }

        Ok((results, reminder))
    }

    /// Returns filtered incoming arcs for a given object node.
    ///
    /// Only includes predicates that appear in the provided allowlist.
    ///
    /// # Arguments
    ///
    /// * `object` - The object term to find incoming relationships for
    /// * `preds` - A slice of predicates to include in the filtered results
    fn incoming_arcs_from_list(
        &self,
        object: &Self::Term,
        preds: &[Self::IRI],
    ) -> Result<IncomingArcs<Self>, Self::Err> {
        let mut results = IncomingArcs::<Self>::new();
        for triple in self.triples_with_object(object)? {
            let (s, p, _) = triple.into_components();
            if preds.contains(&p) {
                results.entry(p).or_default().insert(s);
            }
        }
        Ok(results)
    }

    /// Returns all subjects that are instances of the specified class.
    ///
    /// This method queries for subjects that have `rdf:type` relationships
    /// pointing to the given class term.
    ///
    /// # Arguments
    ///
    /// * `cls` - Matcher for the class (object position of `rdf:type` triples)
    fn shacl_instances_of<O>(&self, cls: &O) -> Result<impl Iterator<Item = Self::Subject>, Self::Err>
    where
        O: Matcher<Self::Term>,
    {
        let rdf_type: Self::IRI = RdfVocab::rdf_type().into();
        let subjects: HashSet<_> = self
            .triples_matching(&Any, &rdf_type, cls)?
            .map(Triple::into_subject)
            .collect();
        Ok(subjects.into_iter())
    }

    /// Returns all subjects that reify the specified triple.
    ///
    /// This method finds RDF reification statements where subjects use
    /// `rdf:reifies` to reference the given triple. This supports RDF-star
    /// reification patterns.
    ///
    /// # Arguments
    ///
    /// * `triple` - The triple to find reifiers for
    fn reifiers_of_triple(&self, triple: &Self::Triple) -> Result<impl Iterator<Item = Self::Subject>, Self::Err> {
        let triple_term = Self::triple_as_term(triple);
        let rdf_reifies: Self::IRI = RdfVocab::rdf_reifies().into();
        let reifiers = Self::triples_with_predicate_object(self, &rdf_reifies, &triple_term)?
            .map(|t| t.into_subject())
            .collect::<HashSet<_>>();
        // Find x such that: x rdf:reifies <<( s p o )>>
        Ok(reifiers.into_iter())
    }

    /// Returns the first object for the given subject-predicate pair.
    ///
    /// This is a convenience method that returns at most one object. If multiple
    /// objects exist, only the first encountered is returned.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject to query
    /// * `predicate` - The predicate to match
    fn object_for(&self, subject: &Self::Term, predicate: &Self::IRI) -> Result<Option<Object>, RDFError> {
        match self.objects_for(subject, predicate)?.into_iter().next() {
            Some(term) => {
                let obj = Self::term_as_object(&term)?;
                Ok(Some(obj))
            },
            None => Ok(None),
        }
    }

    /// Returns all objects reachable by following a SHACL property path.
    ///
    /// SHACL property paths extend simple predicate-based navigation with
    /// complex path expressions including sequences, alternatives, inverses,
    /// and quantifiers.
    ///
    /// # Path Types
    ///
    /// - **Predicate**: Direct predicate navigation (`ex:name`)
    /// - **Alternative**: Union of multiple paths (`ex:father | ex:mother`)
    /// - **Sequence**: Composed paths (`ex:parent / ex:name`)
    /// - **Inverse**: Reverse navigation (`^ex:author`)
    /// - **ZeroOrMore**: Transitive closure (`ex:subClassOf*`)
    /// - **OneOrMore**: Non-empty transitive closure (`ex:subClassOf+`)
    /// - **ZeroOrOne**: Optional path (`ex:nickname?`)
    ///
    /// # Arguments
    ///
    /// * `subject` - The starting term for path navigation
    /// * `path` - The SHACL property path to follow
    fn objects_for_shacl_path(&self, subject: &Self::Term, path: &SHACLPath) -> Result<HashSet<Self::Term>, RDFError> {
        match path {
            SHACLPath::Predicate { pred } => {
                let pred: Self::IRI = pred.clone().into();
                self.objects_for(subject, &pred)
            },
            SHACLPath::Alternative { paths } => {
                let mut all_objects = HashSet::new();
                for path in paths {
                    let objects = self.objects_for_shacl_path(subject, path)?;
                    all_objects.extend(objects);
                }
                Ok(all_objects)
            },
            SHACLPath::Sequence { paths } => match paths.as_slice() {
                [] => Ok(HashSet::from([subject.clone()])),
                [first, rest @ ..] => {
                    let first_objects = self.objects_for_shacl_path(subject, first)?;
                    let mut all_objects = HashSet::new();
                    for obj in first_objects {
                        let intermediate_objects =
                            self.objects_for_shacl_path(&obj, &SHACLPath::Sequence { paths: rest.to_vec() })?;
                        all_objects.extend(intermediate_objects);
                    }
                    Ok(all_objects)
                },
            },
            SHACLPath::Inverse { path } => {
                let pred: Self::IRI = path.pred().unwrap().clone().into();
                let objects = self.subjects_for(&pred, subject)?;
                Ok(objects)
            },
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
            },
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
            },
            SHACLPath::ZeroOrOne { path } => {
                let mut all_objects = HashSet::new();
                all_objects.insert(subject.clone());
                let next_objects = self.objects_for_shacl_path(subject, path)?;
                all_objects.extend(next_objects);
                Ok(all_objects)
            },
        }
    }

    /// Returns all objects for the given subject-predicate pair.
    ///
    /// This method retrieves the object position of all triples matching
    /// the specified subject and predicate.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject term to query
    /// * `predicate` - The predicate IRI to match
    ///
    /// # Errors
    ///
    /// Returns [`RDFError::ErrorObjectsFor`] if the query fails or if the
    /// subject term cannot be converted to a valid subject.
    fn objects_for(&self, subject: &Self::Term, predicate: &Self::IRI) -> Result<HashSet<Self::Term>, RDFError> {
        let subject_node: Self::Subject = Self::term_as_subject(subject)?;
        let triples = self
            .triples_matching(&subject_node, predicate, &Any)
            .map_err(|e| RDFError::ErrorObjectsFor {
                subject: subject.to_string(),
                predicate: predicate.to_string(),
                error: e.to_string(),
            })?
            .map(Triple::into_object)
            .collect();

        Ok(triples)
    }

    /// Returns all subjects for the given predicate-object pair.
    ///
    /// This method performs reverse lookup, finding subjects that have the
    /// specified predicate pointing to the given object.
    ///
    /// # Arguments
    ///
    /// * `predicate` - The predicate IRI to match
    /// * `object` - The object term to query
    ///
    /// # Errors
    ///
    /// Returns [`RDFError::ErrorSubjectsFor`] if the query fails.
    fn subjects_for(&self, predicate: &Self::IRI, object: &Self::Term) -> Result<HashSet<Self::Term>, RDFError> {
        let values = self
            .triples_matching(&Any, predicate, object)
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
