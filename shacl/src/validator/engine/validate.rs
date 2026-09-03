use crate::error::ValidationError;
use crate::ir::{IRSchema, IRShape, ReifierInfo};
use crate::types::MessageMap;
use crate::validator::engine::Engine;
use crate::validator::engine::focus_nodes_ops::FocusNodesOps;
use crate::validator::engine::value_nodes_ops::ValueNodesOps;
use crate::validator::nodes::FocusNodes;
use crate::validator::recursion::cut_outcome;
use crate::validator::report::{ValidationOutcome, ValidationResult};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{NeighsRDF, Rdf, SHACLPath};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

/// Validate RDF data using SHACL
pub trait Validate<RDF: Rdf> {
    fn validate(
        &self,
        store: &RDF,
        runner: &mut dyn Engine<RDF>,
        targets: Option<&FocusNodes<RDF>>,
        source_shape: Option<&IRShape>, // TODO - Review if this is needed since its probably the same as self
        shapes_graph: &IRSchema,
    ) -> Result<ValidationOutcome, ValidationError>;
}

impl<RDF: NeighsRDF + Debug> Validate<RDF> for IRShape {
    fn validate(
        &self,
        store: &RDF,
        runner: &mut dyn Engine<RDF>,
        targets: Option<&FocusNodes<RDF>>,
        source_shape: Option<&IRShape>,
        shapes_graph: &IRSchema,
    ) -> Result<ValidationOutcome, ValidationError> {
        // Skips validation if shape is deactivated
        if self.deactivated() {
            return Ok(ValidationOutcome::new());
        }

        // Get focus nodes
        let focus_nodes = match targets {
            None => &self.focus_nodes(store, runner),
            Some(targets) => targets,
        };

        // Resolve the ShapeLabelIdx for the current shape (used for memoization)
        let idx = shapes_graph.get_idx(self.id());

        // Check the cache: filter out focus nodes that have already been validated
        // and collect their cached outcome. Focus nodes that are already being
        // validated further up this engine's own call stack (a recursive shape
        // reference) are cut with the default outcome for the configured
        // recursion semantics instead of recursing forever — see
        // `crate::validator::recursion`.
        let mut cached_outcome = ValidationOutcome::new();
        let mut entered_chain = Vec::new();
        // Multi-node calls only ever occur for a *batch* of independent
        // targets (this shape's own top-level targets, or several reifiers
        // of one triple) — siblings, not ancestors of each other. Only a
        // single-node call represents one specific node's own place on the
        // recursive-descent path, so only those are chain-eligible: a
        // sibling batch must never poison another sibling by making it look
        // like an ancestor just because they're being resolved together.
        // Every *nested* recursive call in this crate targets exactly one
        // node (`FocusNodes::single`), so this doesn't weaken cycle
        // detection — it just avoids false positives at the batch root.
        let single_node_call = focus_nodes.len() == 1;
        let uncached_focus_nodes = if let Some(idx) = idx {
            let mut uncached = Vec::new();
            for fnode in focus_nodes.iter() {
                let node_object = RDF::term_as_object(fnode);
                let Ok(obj) = node_object else {
                    uncached.push(fnode.clone());
                    continue;
                };
                if let Some(outcome) = runner.get_cached_outcome(&obj, *idx) {
                    cached_outcome.extend(outcome);
                    continue;
                }
                if runner.is_in_chain(&obj, *idx) {
                    cached_outcome.extend(cut_outcome(runner.recursion_semantics(), self, &obj));
                    continue;
                }
                if single_node_call {
                    runner.chain_enter(obj.clone(), *idx);
                    entered_chain.push(obj);
                }
                uncached.push(fnode.clone());
            }
            FocusNodes::from_iter(uncached)
        } else {
            focus_nodes.clone()
        };

        // If all focus nodes were cached (or cut as recursive references), return early
        if uncached_focus_nodes.is_empty() {
            return Ok(cached_outcome);
        }

        // ValueNodes = set of nodes that are going to be used during validation
        // This set of nodes is obtained from the set of (uncached) focus nodes
        let value_nodes = self.value_nodes(store, &uncached_focus_nodes, runner)?;

        let components = self.components();

        // 3. Check each of the components
        let mut component_outcome = ValidationOutcome::new();
        for component in components.iter() {
            let outcome = runner.evaluate(
                store,
                self,
                component,
                &value_nodes,
                source_shape,
                self.path(),
                shapes_graph,
            )?;
            component_outcome.extend(outcome);
        }

        // After validating the constraints of the current shape, validate any nested
        // property shapes.
        //
        // Per the SHACL spec, each value node of the current shape becomes a focus node
        // for the nested property shapes:
        //   - For a NodeShape: value_nodes[F] = {F}, so nested targets == focus nodes.
        //   - For a PropertyShape (path P): value_nodes[F] = objects(F, P), so nested
        //     targets are the nodes reachable via P — NOT the original focus nodes.
        //
        // We iterate per-focus-node (not with a flattened unique set) so that multiplicity
        // is preserved: if the same value node is reached from N different focus nodes,
        // the nested property shape is invoked N times. The shared cache ensures each
        // unique (value-node, shape) pair is only truly validated once; subsequent
        // invocations for the same pair return the cached outcome, correctly producing
        // one violation entry per path that led to the offending node.
        let mut property_shapes_outcome = ValidationOutcome::new();
        for ps in self.property_shapes().iter() {
            let shape = shapes_graph.get_shape_from_idx_e(ps)?;
            for (_, vn) in value_nodes.iter() {
                let outcome = shape.validate(store, runner, Some(vn), Some(self), shapes_graph)?;
                property_shapes_outcome.extend(outcome);
            }
        }

        let reification_outcome = if let Some(reifier_info) = self.reifier_info() {
            validate_reifiers(
                self,
                store,
                runner,
                source_shape,
                reifier_info,
                &uncached_focus_nodes,
                shapes_graph,
            )?
        } else {
            ValidationOutcome::new()
        };

        // Collect all NEW outcome (from uncached focus nodes)
        let mut new_outcome = ValidationOutcome::new();
        new_outcome.extend(component_outcome);
        new_outcome.extend(property_shapes_outcome);
        new_outcome.extend(reification_outcome);

        // These nodes are no longer "in progress": a nested call reaching
        // them now would be a fresh lookup, not a recursive reference.
        if let Some(idx) = idx {
            for obj in &entered_chain {
                runner.chain_exit(obj, *idx);
            }
        }

        // Record new outcome in the cache per focus node
        if let Some(idx) = idx {
            // Group violations/evidences by focus node in O(M), then record each in O(1)
            let mut by_focus: HashMap<Object, ValidationOutcome> = uncached_focus_nodes
                .iter()
                .filter_map(|n| RDF::term_as_object(n).ok())
                .map(|obj| (obj, ValidationOutcome::new()))
                .collect();
            for v in new_outcome.violations() {
                if let Some(bucket) = by_focus.get_mut(v.focus_node()) {
                    bucket.push_violation(v.clone());
                }
            }
            for e in new_outcome.evidences() {
                if let Some(bucket) = by_focus.get_mut(e.focus_node()) {
                    bucket.push_evidence(e.clone());
                }
            }
            for (node_object, node_outcome) in by_focus {
                runner.record_validation(node_object, *idx, node_outcome);
            }
        }

        // Merge cached outcome with newly computed outcome
        cached_outcome.extend(new_outcome);
        Ok(cached_outcome)
    }
}

fn validate_reifiers<RDF: NeighsRDF + Debug>(
    shape: &IRShape,
    store: &RDF,
    runner: &mut dyn Engine<RDF>,
    source_shape: Option<&IRShape>,
    reifier_info: &ReifierInfo,
    focus_nodes: &FocusNodes<RDF>,
    shapes_graph: &IRSchema,
) -> Result<ValidationOutcome, ValidationError> {
    let mut outcome = ValidationOutcome::new();

    for node in focus_nodes.iter() {
        for reifier_shape in reifier_info.reifier_shape() {
            let pred = reifier_info.predicate();
            let pred_iri: RDF::IRI = pred.clone().into();
            let subject = RDF::term_as_subject(node)?;
            let triples = store
                .triples_with_subject_predicate(&subject, &pred_iri)
                .map_err(ValidationError::new_graph_error::<RDF>)?;

            for triple in triples {
                let reifier_subjects = store
                    .reifiers_of_triple(&triple)
                    .map_err(ValidationError::new_graph_error::<RDF>)?
                    .collect::<Vec<_>>();
                if reifier_subjects.is_empty() && reifier_info.reification_required() {
                    let vr_single = ValidationResult::new(
                        shape.id().clone(),
                        Object::iri(ShaclVocab::sh_reifier_shape_constraint_component()),
                        shape.severity().clone(),
                    )
                    .with_message(MessageMap::from(
                        "Reification required but no reifier found for triple {triple} with predicate {pred}",
                    ))
                    .with_path(Some(SHACLPath::iri(pred.clone())))
                    .with_source(source_shape.map(|s| s.id()).cloned());
                    outcome.push_violation(vr_single);
                    continue;
                }
                let reifier_nodes = reifier_subjects
                    .iter()
                    .map(RDF::subject_as_term)
                    .collect::<HashSet<_>>();
                let reifier_shape = shapes_graph.get_shape_from_idx_e(reifier_shape)?;
                let inner_outcome = reifier_shape.validate(
                    store,
                    runner,
                    Some(&FocusNodes::from_iter(reifier_nodes)),
                    Some(shape),
                    shapes_graph,
                )?;
                outcome.extend(inner_outcome)
            }
        }
    }
    Ok(outcome)
}
