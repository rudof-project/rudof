use crate::class_index::ClassIndex;
use crate::constraints::NativeDeref;
use crate::constraints::ShaclComponent;
use crate::focus_nodes::FocusNodes;
use crate::shacl_engine::engine::Engine;
use crate::validate_error::ValidateError;
use crate::validation_cache::ValidationCache;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use iri_s::IriS;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab};
use rudof_rdf::rdf_core::{
    NeighsRDF, SHACLPath,
    term::{Object, Term, Triple},
};
use shacl::ir::{IRComponent, IRSchema, IRShape, ShapeLabelIdx};
use std::fmt::Debug;

pub struct NativeEngine {
    cache: ValidationCache,
    /// Pre-built inverted index mapping classes to their instances and subclasses.
    /// Built once via `build_indexes()` before validation starts.
    class_index: Option<ClassIndex>,
}

impl NativeEngine {
    pub fn new() -> Self {
        Self {
            cache: ValidationCache::new(),
            class_index: None,
        }
    }
}

impl Default for NativeEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: NeighsRDF + Debug + 'static> Engine<S> for NativeEngine {
    fn build_indexes(&mut self, store: &S) -> Result<(), Box<ValidateError>> {
        self.class_index = Some(ClassIndex::build::<S>(store)?);
        Ok(())
    }

    fn evaluate(
        &mut self,
        store: &S,
        shape: &IRShape,
        component: &IRComponent,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, Box<ValidateError>> {
        tracing::debug!("evaluate [NativeEngine] with shape {}", shape.id());
        let shacl_component = ShaclComponent::new(component);
        let validator = shacl_component.deref();
        let result = validator
            .validate_native(
                component,
                shape,
                store,
                self,
                value_nodes,
                source_shape,
                maybe_path,
                shapes_graph,
            )
            .map_err(|e| {
                Box::new(ValidateError::ConstraintError {
                    component: component.to_string(),
                    source: e,
                })
            })?;
        Ok(result)
    }

    /// https://www.w3.org/TR/shacl/#targetNode
    fn target_node(&self, _: &S, node: &Object) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let node: S::Term = node.clone().into();
        if node.is_blank_node() {
            Err(Box::new(ValidateError::TargetNodeBlankNode))
        } else {
            Ok(FocusNodes::single(node.clone()))
        }
    }

    fn target_class(&self, store: &S, class: &Object) -> Result<FocusNodes<S>, Box<ValidateError>> {
        // use the pre-built class index (O(1) lookup)
        if let Some(index) = &self.class_index {
            let focus_nodes = index.instances_of(class).map(|obj| -> S::Term { obj.clone().into() });
            return Ok(FocusNodes::from_iter(focus_nodes));
        }

        // Fallback: full graph scan (for backwards compatibility if index wasn't built)
        let cls: S::Term = class.clone().into();
        let focus_nodes = store
            .shacl_instances_of(&cls)
            .map_err(|e| ValidateError::TargetClassError {
                msg: format!("Failed to get instances of class {class}: {e}"),
            })?
            .map(|subj| S::subject_as_term(&subj));

        Ok(FocusNodes::from_iter(focus_nodes))
    }

    fn target_subject_of(&self, store: &S, predicate: &IriS) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let pred: S::IRI = predicate.clone().into();
        let subjects = store
            .triples_with_predicate(&pred)
            .map_err(|_| ValidateError::SRdf)?
            .map(Triple::into_subject)
            .map(Into::into);
        let focus_nodes = FocusNodes::from_iter(subjects);
        Ok(focus_nodes)
    }

    fn target_object_of(&self, store: &S, predicate: &IriS) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let pred: S::IRI = predicate.clone().into();
        let objects = store
            .triples_with_predicate(&pred)
            .map_err(|_| ValidateError::SRdf)?
            .map(Triple::into_object);
        Ok(FocusNodes::from_iter(objects))
    }

    fn implicit_target_class(&self, store: &S, subject: &Object) -> Result<FocusNodes<S>, Box<ValidateError>> {
        // use the pre-built class index (O(1) lookup)
        if let Some(index) = &self.class_index {
            let instances = index.instances_of_with_subclasses(subject);
            let focus_nodes = instances.into_iter().map(|obj| -> S::Term { obj.into() });
            return Ok(FocusNodes::from_iter(focus_nodes));
        }

        // Fallback: full graph scan (for backwards compatibility if index wasn't built)
        let term: S::Term = subject.clone().into();
        let targets = store
            .subjects_for(&RdfVocab::rdf_type().clone().into(), &term)
            .map_err(|e| ValidateError::InstanceOf {
                term: term.to_string(),
                error: e.to_string(),
            })?;

        let subclass_targets = store
            .subjects_for(&RdfsVocab::rdfs_subclass_of_str().clone().into(), &term)
            .map_err(|e| ValidateError::SubClassOf {
                term: term.to_string(),
                error: e.to_string(),
            })?
            .into_iter()
            .flat_map(move |subclass| {
                store
                    .subjects_for(&RdfVocab::rdf_type().clone().into(), &subclass)
                    .map_err(|e| ValidateError::SubClassOf {
                        term: subclass.to_string(),
                        error: e.to_string(),
                    })
                    .into_iter()
                    .flatten()
            });

        Ok(FocusNodes::from_iter(targets.into_iter().chain(subclass_targets)))
    }

    fn record_validation(&mut self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.cache.record(node, shape_idx, results);
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache.has_validated(node, shape_idx)
    }

    fn get_cached_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<&Vec<ValidationResult>> {
        self.cache.get_results(node, shape_idx)
    }
}
