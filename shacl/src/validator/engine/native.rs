use crate::error::ValidationError;
use crate::ir::{IRComponent, IRSchema, IRShape, ShapeLabelIdx};
use crate::validator::cache::{SharedValidationCache, ValidationCache};
use crate::validator::constraints::{NativeValidator, ShaclComponent, ValidatorDeref};
use crate::validator::engine::Engine;
use crate::validator::index::ClassIndex;
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::{Object, Term, Triple};
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;
use std::sync::Arc;

#[cfg(feature = "sparql")]
use crate::validator::constraints::BasicSparqlValidator;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::query::QueryRDF;

pub struct NativeEngine {
    /// Shared, thread-safe validation cache.
    cache: SharedValidationCache,
    /// Pre-built inverted index mapping classes to their instances and subclasses.
    class_index: Option<Arc<ClassIndex>>,
}

impl NativeEngine {
    pub fn new() -> Self {
        Self {
            cache: SharedValidationCache::new(),
            class_index: None,
        }
    }

    fn build_indexes_impl<RDF: NeighsRDF + Debug + 'static>(&mut self, store: &RDF) -> Result<(), ValidationError> {
        self.class_index = Some(Arc::new(ClassIndex::build(store)?));
        Ok(())
    }

    fn target_node_impl<RDF: NeighsRDF + Debug + 'static>(&self, node: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        let node: RDF::Term = node.clone().into();
        if node.is_blank_node() {
            Err(ValidationError::TargetNodeBNode)
        } else {
            Ok(FocusNodes::single(node.clone()))
        }
    }

    fn target_class_impl<RDF: NeighsRDF + Debug + 'static>(
        &self,
        store: &RDF,
        class: &Object,
    ) -> Result<FocusNodes<RDF>, ValidationError> {
        if let Some(index) = &self.class_index {
            let focus_nodes = index.instances_of(class).map(|obj| -> RDF::Term { obj.clone().into() });
            return Ok(FocusNodes::from_iter(focus_nodes));
        }

        let cls: RDF::Term = class.clone().into();
        let focus_nodes = store
            .shacl_instances_of(&cls)
            .map_err(ValidationError::new_graph_error::<RDF>)?
            .map(|s| RDF::subject_as_term(&s));

        Ok(FocusNodes::from_iter(focus_nodes))
    }

    fn target_subject_of_impl<RDF: NeighsRDF + Debug + 'static>(
        &self,
        store: &RDF,
        predicate: &IriS,
    ) -> Result<FocusNodes<RDF>, ValidationError> {
        let pred: RDF::IRI = predicate.clone().into();
        let subjects = store
            .triples_with_predicate(&pred)
            .map_err(ValidationError::new_graph_error::<RDF>)?
            .map(Triple::into_subject)
            .map(Into::into);
        Ok(FocusNodes::from_iter(subjects))
    }

    fn target_object_of_impl<RDF: NeighsRDF + Debug + 'static>(
        &self,
        store: &RDF,
        predicate: &IriS,
    ) -> Result<FocusNodes<RDF>, ValidationError> {
        let pred: RDF::IRI = predicate.clone().into();
        let objects = store
            .triples_with_predicate(&pred)
            .map_err(ValidationError::new_graph_error::<RDF>)?
            .map(Triple::into_object);
        Ok(FocusNodes::from_iter(objects))
    }

    fn implicit_target_class_impl<RDF: NeighsRDF + Debug + 'static>(
        &self,
        store: &RDF,
        shape: &Object,
    ) -> Result<FocusNodes<RDF>, ValidationError> {
        if let Some(index) = &self.class_index {
            let instances = index.instances_of_with_subclasses(shape);
            let focus_nodes = instances.into_iter().map(|obj| -> RDF::Term { obj.clone().into() });
            return Ok(FocusNodes::from_iter(focus_nodes));
        }

        let term: RDF::Term = shape.clone().into();
        let targets = store.subjects_for(&RdfVocab::rdf_type().into(), &term)?;

        let subclass_targets = store
            .subjects_for(&RdfsVocab::rdfs_subclass_of_str().into(), &term)?
            .into_iter()
            .flat_map(move |subclass| {
                store
                    .subjects_for(&RdfVocab::rdf_type().into(), &subclass)
                    .into_iter()
                    .flatten()
            });

        Ok(FocusNodes::from_iter(targets.into_iter().chain(subclass_targets)))
    }
}

#[cfg(not(feature = "sparql"))]
impl<RDF: NeighsRDF + Debug + 'static> Engine<RDF> for NativeEngine {
    fn build_indexes(&mut self, store: &RDF) -> Result<(), ValidationError> {
        self.build_indexes_impl(store)
    }

    fn fork(&self) -> Box<dyn Engine<RDF>> {
        Box::new(NativeEngine {
            cache: self.cache.clone(),
            class_index: self.class_index.clone(),
        })
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
        validator.validate_native(
            component,
            shape,
            store,
            self,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }

    fn target_node(&self, _: &RDF, node: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        self.target_node_impl(node)
    }

    fn target_class(&self, store: &RDF, class: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        self.target_class_impl(store, class)
    }

    fn target_subject_of(&self, store: &RDF, predicate: &IriS) -> Result<FocusNodes<RDF>, ValidationError> {
        self.target_subject_of_impl(store, predicate)
    }

    fn target_object_of(&self, store: &RDF, predicate: &IriS) -> Result<FocusNodes<RDF>, ValidationError> {
        self.target_object_of_impl(store, predicate)
    }

    fn implicit_target_class(&self, store: &RDF, shape: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        self.implicit_target_class_impl(store, shape)
    }

    fn record_validation(&mut self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.cache.record(node, shape_idx, results);
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache.has_validated(node, shape_idx)
    }

    fn get_cached_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<Vec<ValidationResult>> {
        self.cache.get_results(node, shape_idx)
    }
}

#[cfg(feature = "sparql")]
impl<RDF: NeighsRDF + QueryRDF + Debug + 'static> Engine<RDF> for NativeEngine {
    fn build_indexes(&mut self, store: &RDF) -> Result<(), ValidationError> {
        self.build_indexes_impl(store)
    }

    fn fork(&self) -> Box<dyn Engine<RDF>> {
        Box::new(NativeEngine {
            cache: self.cache.clone(),
            class_index: self.class_index.clone(),
        })
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
        if let IRComponent::BasicSparql(basic_sparql) = component {
            return basic_sparql.validate_sparql(
                component,
                shape,
                store,
                self,
                value_nodes,
                source_shape,
                maybe_path,
                shapes_graph,
            );
        }

        let shacl_component = ShaclComponent::new(component);
        let validator: &dyn NativeValidator<RDF> = shacl_component.deref();
        validator.validate_native(
            component,
            shape,
            store,
            self,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }

    fn target_node(&self, _: &RDF, node: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        self.target_node_impl(node)
    }

    fn target_class(&self, store: &RDF, class: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        self.target_class_impl(store, class)
    }

    fn target_subject_of(&self, store: &RDF, predicate: &IriS) -> Result<FocusNodes<RDF>, ValidationError> {
        self.target_subject_of_impl(store, predicate)
    }

    fn target_object_of(&self, store: &RDF, predicate: &IriS) -> Result<FocusNodes<RDF>, ValidationError> {
        self.target_object_of_impl(store, predicate)
    }

    fn implicit_target_class(&self, store: &RDF, shape: &Object) -> Result<FocusNodes<RDF>, ValidationError> {
        self.implicit_target_class_impl(store, shape)
    }

    fn record_validation(&mut self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.cache.record(node, shape_idx, results);
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache.has_validated(node, shape_idx)
    }

    fn get_cached_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<Vec<ValidationResult>> {
        self.cache.get_results(node, shape_idx)
    }
}

impl Default for NativeEngine {
    fn default() -> Self {
        Self::new()
    }
}
