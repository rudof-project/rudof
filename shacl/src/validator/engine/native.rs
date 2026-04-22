use crate::error::ValidationError;
use crate::ir::{IRComponent, IRSchema, IRShape, ShapeLabelIdx};
use crate::validator::cache::ValidationCache;
use crate::validator::constraints::{NativeValidator, ShaclComponent, ValidatorDeref};
use crate::validator::engine::Engine;
use crate::validator::index::ClassIndex;
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;
use iri_s::IriS;
use rudof_rdf::rdf_core::term::{Object, Term, Triple};
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

pub struct NativeEngine {
    cache: ValidationCache,
    /// Pre-built inverted index mapping classes to their instances and subclasses
    /// Built once via `build_indexes()` before validation starts
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

impl<RDF: NeighsRDF + Debug + 'static> Engine<RDF> for NativeEngine {
    fn build_indexes(&mut self, store: &RDF) -> Result<(), ValidationError> {
        self.class_index = Some(ClassIndex::build(store)?);
        Ok(())
    }

    fn evaluate(
        &mut self,
        store: &RDF,
        shape: &IRShape,
        component: &IRComponent,
        value_nodes: &ValueNodes<RDF>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        let shacl_component = ShaclComponent::new(component);
        let validator: &dyn NativeValidator<RDF> = shacl_component.deref();

        validator
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
            .map_err(|e| ValidationError::ConstraintError {
                component: component.to_string(),
                source: Box::new(e),
            })
    }

    /// https://www.w3.org/TR/shacl/#targetNode
    fn target_node(&self, _: &RDF, node: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        let node: RDF::Term = node.clone().into();
        if node.is_blank_node() {
            Err(ValidationError::TargetNodeBNode)
        } else {
            Ok(FocusNodes::single(node.clone()))
        }
    }

    fn target_class(&self, store: &RDF, class: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        // use the pre-built class index (O(1) lookup)
        if let Some(index) = &self.class_index {
            let focus_nodes = index.instances_of(class).map(|obj| -> RDF::Term { obj.clone().into() });
            return Ok(FocusNodes::from_iter(focus_nodes));
        }

        // Fallback: full graph scan (for backwards compatibility if index wasn't built)
        let cls: RDF::Term = class.clone().into();
        let focus_nodes = store
            .shacl_instances_of(&cls)
            .map_err(|e| ValidationError::TargetClassError {
                msg: format!("Failed to get instances of class {class}: {e}"),
            })?
            .map(|s| RDF::subject_as_term(&s));

        Ok(FocusNodes::from_iter(focus_nodes))
    }

    fn target_subject_of(&self, store: &RDF, predicate: &IriS) -> Result<FocusNodes<RDF>, ValidationError> {
        let pred: RDF::IRI = predicate.clone().into();
        let subjects = store
            .triples_with_predicate(&pred)
            .map_err(|_| ValidationError::Srdf)?
            .map(Triple::into_subject)
            .map(Into::into);
        Ok(FocusNodes::from_iter(subjects))
    }

    fn target_object_of(&self, store: &RDF, predicate: &IriS) -> Result<FocusNodes<RDF>, ValidationError> {
        let pred: RDF::IRI = predicate.clone().into();
        let objects = store
            .triples_with_predicate(&pred)
            .map_err(|_| ValidationError::Srdf)?
            .map(Triple::into_object);
        Ok(FocusNodes::from_iter(objects))
    }

    fn implicit_target_class(&self, store: &RDF, shape: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        // use the pre-built class index (O(1) lookup)
        if let Some(index) = &self.class_index {
            let instances = index.instances_of_with_subclasses(shape);
            let focus_nodes = instances.into_iter().map(|obj| -> RDF::Term { obj.clone().into() });
            return Ok(FocusNodes::from_iter(focus_nodes));
        }

        // Fallback: full graph scan (for backwards compatibility if index wasn't built)
        let term: RDF::Term = shape.clone().into();
        let targets =
            store
                .subjects_for(&RdfVocab::rdf_type().into(), &term)
                .map_err(|e| ValidationError::InstanceOf {
                    term: term.to_string(),
                    error: e.to_string(),
                })?;

        let subclass_targets = store
            .subjects_for(&RdfsVocab::rdfs_subclass_of_str().into(), &term)
            .map_err(|e| ValidationError::SubClassOf {
                term: term.to_string(),
                error: e.to_string(),
            })?
            .into_iter()
            .flat_map(move |subclass| {
                store
                    .subjects_for(&RdfVocab::rdf_type().into(), &subclass)
                    .map_err(|e| ValidationError::SubClassOf {
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

impl Default for NativeEngine {
    fn default() -> Self {
        Self::new()
    }
}
