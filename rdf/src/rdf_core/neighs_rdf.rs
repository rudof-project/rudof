use std::{collections::{HashSet, HashMap}, vec::IntoIter};
use crate::rdf_core::{
    Rdf, RDFError, Matcher, Any, SHACLPath,
    term::{Object, Triple},
    vocab::{rdf_reifies, rdf_type}
};

//----------------------------------------------------------------
// Type aliases for common RDF navigation patterns
//----------------------------------------------------------------

/// Maps predicates to sets of subjects (inverse navigation)
pub type IncomingArcs<R> = HashMap<<R as Rdf>::IRI, HashSet<<R as Rdf>::Subject>>;
/// Maps predicates to sets of objects (forward navigation)
pub type OutgoingArcs<R> = HashMap<<R as Rdf>::IRI, HashSet<<R as Rdf>::Term>>;
/// Filtered outgoing arcs with remainder predicates
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
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err>
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
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
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
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
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
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
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
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        self.triples_matching(&Any, predicate, object)
    }

    /// Returns all triples with the specified object.
    ///
    /// Equivalent to `triples_matching(Any, Any, object)`.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to match
    fn triples_with_object(
        &self,
        object: &Self::Term,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
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
        let mut remainder = Vec::new();

        for triple in self.triples_with_subject(subject)? {
            let (_, p, o) = triple.into_components();
            if preds.contains(&p) {
                results.entry(p).or_default().insert(o);
            } else {
                remainder.push(p)
            }
        }

        Ok((results, remainder))
    }

    /// Returns all subjects that are instances of the specified class.
    ///
    /// This method queries for subjects that have `rdf:type` relationships
    /// pointing to the given class term.
    ///
    /// # Arguments
    ///
    /// * `cls` - Matcher for the class (object position of `rdf:type` triples)
    fn shacl_instances_of<O>(
        &self,
        cls: &O,
    ) -> Result<impl Iterator<Item = Self::Subject>, Self::Err>
    where
        O: Matcher<Self::Term>,
    {
        let rdf_type: Self::IRI = rdf_type().clone().into();
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
    fn reifiers_of_triple(
        &self,
        triple: &Self::Triple,
    ) -> Result<impl Iterator<Item = Self::Subject>, Self::Err> {
        let triple_term = Self::triple_as_term(triple);
        let rdf_reifies: Self::IRI = rdf_reifies().clone().into();
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
                let pred: Self::IRI = path.pred().unwrap().clone().into();
                let objects = self.subjects_for(&pred, subject)?;
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
    fn objects_for(
        &self,
        subject: &Self::Term,
        predicate: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, RDFError> {
        let subject_node: Self::Subject = Self::term_as_subject(subject)?;
        let subject_str = format!("{subject}");
        let predicate_str = format!("{predicate}");
        let triples = self
            .triples_matching(&subject_node, predicate, &Any)
            .map_err(|e| RDFError::ErrorObjectsFor {
                subject: subject_str,
                predicate: predicate_str,
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
    fn subjects_for(
        &self,
        predicate: &Self::IRI,
        object: &Self::Term,
    ) -> Result<HashSet<Self::Term>, RDFError> {
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

/// Represents a single neighborhood relationship in an RDF graph.
///
/// A neighborhood relationship can be either a direct connection (outgoing arc)
/// or an inverse connection (incoming arc) relative to a focus node.
///
/// # Type Parameters
///
/// * `S` - The RDF graph type implementing [`NeighsRDF`]
///
/// # Variants
///
/// - [`Direct`](Self::Direct): An outgoing relationship where the focus node is the subject
/// - [`Inverse`](Self::Inverse): An incoming relationship where the focus node is the object
pub enum Neigh<S>
where
    S: NeighsRDF,
{
    /// A direct (outgoing) relationship from the focus node.
    ///
    /// Represents a triple pattern: `(focusNode, p, o)` where the focus node
    /// is the subject, `p` is the predicate, and `o` is the object.
    ///
    /// # Fields
    ///
    /// * `p` - The predicate IRI of the relationship
    /// * `o` - The object term that the predicate points to
    Direct { p: S::IRI, o: S::Term },
    /// An inverse (incoming) relationship to the focus node.
    ///
    /// Represents a triple pattern: `(s, p, focusNode)` where `s` is the subject,
    /// `p` is the predicate, and the focus node is the object.
    ///
    /// # Fields
    ///
    /// * `s` - The subject that has this relationship to the focus node
    /// * `p` - The predicate IRI of the relationship
    Inverse { s: S::Subject, p: S::IRI },
}

impl<S> Neigh<S>
where
    S: NeighsRDF,
{
    /// Creates a direct (outgoing) neighborhood relationship.
    ///
    /// Constructs a neighborhood representing an outgoing arc from a focus node
    /// via the specified predicate to the given object.
    ///
    /// # Arguments
    ///
    /// * `pred` - The predicate IRI of the relationship
    /// * `object` - The object term that the predicate points to
    pub fn direct(pred: S::IRI, object: S::Term) -> Neigh<S> {
        Neigh::Direct { p: pred, o: object }
    }

    /// Creates an inverse (incoming) neighborhood relationship.
    ///
    /// Constructs a neighborhood representing an incoming arc to a focus node
    /// via the specified predicate from the given subject.
    ///
    /// # Arguments
    ///
    /// * `pred` - The predicate IRI of the relationship
    /// * `subject` - The subject that has this relationship to the focus node
    pub fn inverse(pred: S::IRI, subject: S::Subject) -> Neigh<S> {
        Neigh::Inverse {
            p: pred,
            s: subject,
        }
    }
}

/// An iterator over the neighborhood of a node in an RDF graph.
///
/// This lazy iterator yields all direct (outgoing) and inverse (incoming) relationships
/// for a given node in an RDF graph without materializing the entire neighborhood in memory at once.
///
/// # Type Parameters
///
/// * `S` - The RDF graph type implementing [`NeighsRDF`]
pub struct NeighsIterator<S>
where
    S: NeighsRDF,
{
    /// The term whose neighborhood is being iterated over.
    _term: S::Term,

    /// Internal iterator over neighborhood relationships [`Neigh`].
    _neigh_iter: IntoIter<Neigh<S>>,
}

impl<S> NeighsIterator<S>
where
    S: NeighsRDF,
{
    /// Creates a new neighborhood iterator for the given term.
    ///
    /// This method initializes an iterator that will traverse all direct and
    /// inverse relationships of the specified term in the RDF graph.
    ///
    /// # Arguments
    ///
    /// * `term` - The RDF term whose neighborhood should be iterated
    /// * `rdf` - The RDF graph to query for neighborhood relationships
    pub fn new(term: S::Term, rdf: S) -> Result<NeighsIterator<S>, S::Err> {
        match S::term_as_subject(&term) {
            Ok(subject) => {
                let subject: S::Subject = subject;
                // Collect all predicates for this subject
                let preds: HashSet<S::IRI> = rdf
                    .triples_with_subject(&subject)?
                    .map(Triple::into_predicate)
                    .collect();
                let _qs = preds.into_iter();
                
                // TODO: Complete implementation
                // The intended approach is to:
                // 1. For each predicate, get all objects (direct neighs)
                // 2. Collect predicates where term appears as object (inverse neighs)
                // 3. Create a lazy iterator that yields both types of neighs
                
                /*let vv = qs.flat_map(|p| {
                    let objs = rdf.get_objects_for_subject_predicate(&subject, &p)?;
                    objs.into_iter().map(|o| Neigh::Direct { p, o })
                });*/
                
                todo!(); // Ok(vv)
            }
            Err(_) => {
                // TODO: Handle case where term is not a subject
                // Should still find inverse relationships where term appears as object
                todo!()
            }
        }
    }
}

impl<S> FromIterator<Neigh<S>> for NeighsIterator<S>
where
    S: NeighsRDF,
{
    /// Constructs a `NeighsIterator` from an iterator of neighborhoods.
    fn from_iter<T>(_t: T) -> Self
    where
        T: IntoIterator,
    {
        todo!()
    }
}

impl<S> Iterator for NeighsIterator<S>
where
    S: NeighsRDF,
{
    /// The neighborhood relationship type yielded by this iterator.
    type Item = Neigh<S>;

    /// Advances the iterator and returns the next neighborhood relationship.
    ///
    /// # Returns
    ///
    /// - `Some(Neigh<S>)` if there are more neighborhood relationships
    /// - `None` if the iteration is complete
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
