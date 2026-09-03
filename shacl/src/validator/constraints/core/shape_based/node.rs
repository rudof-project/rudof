use crate::error::ValidationError;
use crate::ir::components::Node;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::Validator;
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Node {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<ValidationOutcome, ValidationError> {
        let mut outcome = ValidationOutcome::new();
        let shape_idx = self.shape();
        let node_shape = shapes_graph.get_shape_from_idx_e(shape_idx)?;
        let component_obj = Object::iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            for node in nodes.iter() {
                let node_object = S::term_as_object(node)?;
                let focus_nodes = FocusNodes::single(node.clone());

                let had_violations = if engine.has_validated(&node_object, *shape_idx) {
                    engine
                        .get_cached_outcome(&node_object, *shape_idx)
                        .map(|r| !r.conforms())
                        .unwrap_or(false)
                } else {
                    let inner_results =
                        node_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                    !inner_results?.conforms()
                };

                if had_violations {
                    let msg = format!(
                        "Shape {}: Node({node_shape}) constraint not satisfied for {node}",
                        shape.id()
                    );
                    let vr = ValidationResult::new(fnode_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_path(maybe_path.cloned())
                        .with_message(MessageMap::from(msg))
                        .with_value(Some(node_object.clone()))
                        .with_source(Some(shape.id().clone()));
                    outcome.push_violation(vr);
                } else {
                    let ev = Evidence::new(fnode_obj.clone(), component_obj.clone())
                        .with_path(maybe_path.cloned())
                        .with_value(Some(node_object.clone()))
                        .with_source(Some(shape.id().clone()));
                    outcome.push_evidence(ev);
                }
            }
        }

        Ok(outcome)
    }
}
