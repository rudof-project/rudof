use crate::error::ValidationError;
use crate::ir::components::QualifiedValueShape;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::Validator;
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::collections::HashSet;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for QualifiedValueShape {
    fn validate(
        &self,
        _: &IRComponent,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<ValidationOutcome, ValidationError> {
        // TODO - It works but it returns duplicated validation results
        // I tried to use a HashSet but it still doesn't remove duplicates...
        let mut validation_results = HashSet::new();
        let mut evidences = Vec::new();

        for (fnode, nodes) in value_nodes.iter() {
            let mut valid_counter = 0;
            let fnode_obj = S::term_as_object(fnode)?;
            // Count how many nodes conform to the shape
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let qv_shape = shapes_graph.get_shape_from_idx_e(self.shape())?;
                let inner_results = qv_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                let mut is_valid = match inner_results {
                    Ok(results) => results.conforms(),
                    Err(_) => false,
                };

                if !self.siblings().is_empty() && is_valid {
                    // If there are siblings, check that none of them validate
                    for sibling in self.siblings().iter() {
                        let sibling_shape = shapes_graph.get_shape_from_idx_e(sibling)?;
                        let sibling_results =
                            sibling_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                        let sibling_is_valid = sibling_results.is_ok_and(|r| r.conforms());
                        if sibling_is_valid {
                            is_valid = false;
                            break;
                        }
                    }
                }

                if is_valid {
                    valid_counter += 1;
                }
            }

            if let Some(min_count) = self.qualified_min_count() {
                let component = Object::iri(ShaclVocab::sh_qualified_min_count_constraint_component());
                if valid_counter < min_count {
                    let msg = format!(
                        "QualifiedValueShape: only {valid_counter} nodes conform to shape {}, which is less than minCount: {min_count}. Focus node: {fnode}",
                        shape.id()
                    );
                    let vr = ValidationResult::new(fnode_obj.clone(), component, shape.severity().clone())
                        .with_message(MessageMap::from(msg))
                        .with_path(maybe_path.cloned())
                        .with_source(Some(shape.id().clone()));
                    validation_results.insert(vr);
                } else {
                    evidences.push(
                        Evidence::new(fnode_obj.clone(), component)
                            .with_path(maybe_path.cloned())
                            .with_source(Some(shape.id().clone())),
                    );
                }
            }

            if let Some(max_count) = self.qualified_max_count() {
                let component = Object::iri(ShaclVocab::sh_qualified_max_count_constraint_component());
                if valid_counter > max_count {
                    let msg = format!(
                        "QualifiedValueShape: {valid_counter} nodes conform to shape {}, which is grater than maxCount: {max_count}. Focus node: {fnode}",
                        shape.id()
                    );
                    let vr = ValidationResult::new(fnode_obj.clone(), component, shape.severity().clone())
                        .with_path(maybe_path.cloned())
                        .with_message(MessageMap::from(msg))
                        .with_source(Some(shape.id().clone()));
                    validation_results.insert(vr);
                } else {
                    evidences.push(
                        Evidence::new(fnode_obj, component)
                            .with_path(maybe_path.cloned())
                            .with_source(Some(shape.id().clone())),
                    );
                }
            }
        }

        let mut outcome = ValidationOutcome::from_violations(validation_results.into_iter().collect());
        for ev in evidences {
            outcome.push_evidence(ev);
        }
        Ok(outcome)
    }
}
