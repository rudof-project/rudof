use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_core::term::{Object, Triple};
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab};
use std::collections::{HashMap, HashSet};

use crate::validate_error::ValidateError;

/// Pre-computed inverted index for class-based target resolution.
///
/// Instead of scanning all `rdf:type` triples every time `target_class` or
/// `implicit_target_class` is called, this index is built **once** before
/// validation starts, turning the per-shape O(n) scan into a single O(n)
/// build step followed by O(1) lookups per class.
///
/// The index stores:
/// - `class_instances`: `class_object → { subject_as_term, … }` built from all
///   `?s rdf:type ?class` triples.
/// - `subclass_map`: `class_object → { subclass_as_term, … }` built from all
///   `?sub rdfs:subClassOf ?class` triples (one level).
#[derive(Debug, Clone, Default)]
pub struct ClassIndex {
    /// Maps each class to the set of its direct `rdf:type` instances.
    class_instances: HashMap<Object, HashSet<Object>>,
    /// Maps each class to the set of its direct `rdfs:subClassOf` subclasses.
    subclass_map: HashMap<Object, HashSet<Object>>,
}

impl ClassIndex {
    /// Builds both indexes by performing a single pass over all triples.
    pub fn build<S: NeighsRDF>(store: &S) -> Result<Self, Box<ValidateError>> {
        let mut class_instances: HashMap<Object, HashSet<Object>> = HashMap::new();
        let mut subclass_map: HashMap<Object, HashSet<Object>> = HashMap::new();

        let rdf_type: S::IRI = RdfVocab::rdf_type().into();
        let rdfs_subclass_of: S::IRI = RdfsVocab::rdfs_subclass_of_str().into();

        for triple in store
            .triples()
            .map_err(|e| ValidateError::ClassIndexBuild { error: e.to_string() })?
        {
            let (subj, pred, obj) = triple.into_components();

            if pred == rdf_type {
                // ?subj rdf:type ?obj  →  obj is a class, subj is an instance
                let class_obj = S::term_as_object(&obj).ok();
                let instance_obj = S::term_as_object(&S::subject_as_term(&subj)).ok();
                if let (Some(cls), Some(inst)) = (class_obj, instance_obj) {
                    class_instances.entry(cls).or_default().insert(inst);
                }
            } else if pred == rdfs_subclass_of {
                // ?subj rdfs:subClassOf ?obj  →  subj is subclass of obj
                let parent_obj = S::term_as_object(&obj).ok();
                let child_obj = S::term_as_object(&S::subject_as_term(&subj)).ok();
                if let (Some(parent), Some(child)) = (parent_obj, child_obj) {
                    subclass_map.entry(parent).or_default().insert(child);
                }
            }
        }

        Ok(ClassIndex {
            class_instances,
            subclass_map,
        })
    }

    /// Returns the set of direct instances of the given class.
    pub fn instances_of(&self, class: &Object) -> impl Iterator<Item = &Object> {
        self.class_instances.get(class).into_iter().flat_map(|set| set.iter())
    }

    /// Returns the set of direct subclasses of the given class.
    pub fn subclasses_of(&self, class: &Object) -> impl Iterator<Item = &Object> {
        self.subclass_map.get(class).into_iter().flat_map(|set| set.iter())
    }

    /// Returns instances of the class and instances of all its direct subclasses.
    pub fn instances_of_with_subclasses(&self, class: &Object) -> HashSet<Object> {
        let mut result: HashSet<Object> = self.instances_of(class).cloned().collect();
        for subclass in self.subclasses_of(class) {
            for instance in self.instances_of(subclass) {
                result.insert(instance.clone());
            }
        }
        result
    }
}
