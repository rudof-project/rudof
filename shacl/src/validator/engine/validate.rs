use std::collections::HashSet;
use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, Rdf, SHACLPath};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use crate::error::ValidationError;
use crate::ir::{IRSchema, IRShape, ReifierInfo};
use crate::types::MessageMap;
use crate::validator::constraints::get_shape_from_idx;
use crate::validator::engine::Engine;
use crate::validator::engine::focus_nodes_ops::FocusNodesOps;
use crate::validator::engine::value_nodes_ops::ValueNodesOps;
use crate::validator::nodes::FocusNodes;
use crate::validator::report::ValidationResult;

/// Validate RDF data using SHACL
pub trait Validate<RDF: Rdf> {
    fn validate(
        &self,
        store: &RDF,
        runner: &mut dyn Engine<RDF>,
        targets: Option<&FocusNodes<RDF>>,
        source_shape: Option<&IRShape>, // TODO - Review if this is needed since its probably the same as self
        shapes_graph: &IRSchema
    ) -> Result<Vec<ValidationResult>, ValidationError>;
}

impl<RDF: NeighsRDF + Debug> Validate<RDF> for IRShape {
    fn validate(&self, store: &RDF, runner: &mut dyn Engine<RDF>, targets: Option<&FocusNodes<RDF>>, source_shape: Option<&IRShape>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ValidationError> {
        // Skips validation if shape is deactivated
        if self.deactivated() {
            return Ok(Vec::new())
        }

        // Get focus nodes
        let focus_nodes = match targets {
            None => &self.focus_nodes(store, runner),
            Some(targets) => targets,
        };

        // Resolve the ShapeLabelIdx for the current shape (used for memoization)
        let idx = shapes_graph.get_idx(self.id());

        // Check the cache: filter out focus nodes that have already been validated
        // and collect their cached results
        let mut cached_results = Vec::new();
        let uncached_focus_nodes = if let Some(idx) = idx {
            let mut uncached = Vec::new();
            for fnode in focus_nodes.iter() {
                let node_object = RDF::term_as_object(fnode);
                if let Ok(ref obj) = node_object
                    && let Some(results) = runner.get_cached_results(obj, *idx) {
                    cached_results.extend(results.iter().cloned());
                    continue;
                }
                uncached.push(fnode.clone());
            }
            FocusNodes::from_iter(uncached)
        } else {
            focus_nodes.clone()
        };

        // If all focus nodes were cached, return early
        if uncached_focus_nodes.is_empty() {
            return Ok(cached_results);
        }

        // ValueNodes = set of nodes that are going to be used during validation
        // This set of nodes is obtained from the set of (uncached) focus nodes
        let value_nodes = self.value_nodes(store, &uncached_focus_nodes, runner)?;

        let components = self.components();

        // 3. Check each of the components
        let mut component_validation_results = Vec::new();
        for component in components.iter() {
            let results = runner.evaluate(
                store,
                self,
                component,
                &value_nodes,
                source_shape,
                self.path(),
                shapes_graph,
            )?;
            component_validation_results.extend(results);
        }

        // After validating the constraints that are defined in the current Shape,
        // it is important to also perform the validation over those nested PropertyShapes.
        // The validation needs to occur over the focus_nodes that have been computed for the current shape
        let mut property_shapes_validation_results = Vec::new();
        for ps in self.property_shapes().iter() {
            let shape = shapes_graph.get_shape_from_idx(ps).unwrap_or_else(|| {
                panic!("Internal error: Property shape for idx: {ps} not found in schema")
            });
            let results = shape.validate(store, runner, Some(&uncached_focus_nodes), Some(self), shapes_graph)?;
            property_shapes_validation_results.extend(results);
        }

        let reification_results = if let Some(reifier_info) = self.reifier_info() {
            validate_reifiers(
                self,
                store,
                runner,
                source_shape,
                reifier_info,
                &uncached_focus_nodes,
                shapes_graph
            )?
        } else {
            Vec::new()
        };

        // Collect all NEW validation results (from uncached focus nodes)
        let new_results: Vec<ValidationResult> = component_validation_results
            .into_iter()
            .chain(property_shapes_validation_results)
            .chain(reification_results)
            .collect();

        // Record new results in the cache per focus node
        if let Some(idx) = idx {
            // Group results by focus node for caching
            for focus_node in uncached_focus_nodes.iter() {
                if let Ok(node_object) = RDF::term_as_object(focus_node) {
                    let node_results = new_results
                        .iter()
                        .filter(|r| r.focus_node() == &node_object)
                        .cloned()
                        .collect();
                    runner.record_validation(node_object, *idx, node_results);
                }
            }
        }

        // Merge cached results with newly computed results
        let mut all_results = cached_results;
        all_results.extend(new_results);
        Ok(all_results)
    }
}

fn validate_reifiers<RDF: NeighsRDF + Debug>(shape: &IRShape, store: &RDF, runner: &mut dyn Engine<RDF>, source_shape: Option<&IRShape>, reifier_info: &ReifierInfo, focus_nodes: &FocusNodes<RDF>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ValidationError> {
    let mut results = Vec::new();

    for node in focus_nodes.iter() {
        for reifier_shape in reifier_info.reifier_shape() {
            let pred = reifier_info.predicate();
            let pred_iri: RDF::IRI = pred.clone().into();
            let subject = RDF::term_as_subject(node).map_err(|_| ValidationError::TriplesWithSubject {
                subject: format!("{node:?}"),
                error: "Cannot convert to subject".to_string(),
            })?;
            let triples = store.triples_with_subject_predicate(&subject, &pred_iri).map_err(|e| ValidationError::TriplesWithSubjectPredicate {
                subject: node.to_string(),
                predicate: pred.to_string(),
                error: e.to_string(),
            })?;

            for triple in triples {
                let reifier_subjects = store
                    .reifiers_of_triple(&triple)
                    .map_err(|e| ValidationError::ReifiersOfTriple {
                        triple: format!("{triple:?}"),
                        error: e.to_string(),
                    })?
                    .collect::<Vec<_>>();
                if reifier_subjects.is_empty() && reifier_info.reification_required() {
                    let vr_single = ValidationResult::new(
                        shape.id().clone(),
                        Object::iri(ShaclVocab::sh_reifier_shape_constraint_component()),
                        shape.severity(),
                    )
                        .with_message(MessageMap::from("Reification required but no reifier found for triple {triple} with predicate {pred}"))
                        .with_path(Some(SHACLPath::iri(pred.clone())))
                        .with_source(source_shape.map(|s| s.id()).cloned());
                    results.push(vr_single);
                    continue;
                }
                let reifier_nodes = reifier_subjects
                    .iter()
                    .map(RDF::subject_as_term)
                    .collect::<HashSet<_>>();
                let reifier_shape = get_shape_from_idx(shapes_graph, reifier_shape).map_err(|e| ValidationError::ShapeNotFound {
                    idx: *reifier_shape,
                    error: e.to_string(),
                })?;
                let vr_iter = reifier_shape.validate(
                    store,
                    runner,
                    Some(&FocusNodes::from_iter(reifier_nodes)),
                    Some(shape),
                    shapes_graph,
                )?;
                results.extend(vr_iter.into_iter())
            }
        }
    }
    Ok(results)
}