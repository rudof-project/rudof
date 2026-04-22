use crate::ir::components::Node;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::{ConstraintError, Validator, get_shape_from_idx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;
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
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();
        let shape_idx = self.shape();
        let node_shape = get_shape_from_idx(shapes_graph, shape_idx)?;
        let component_obj = Object::iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            for node in nodes.iter() {
                let node_object = S::term_as_object(node)?;
                let focus_nodes = FocusNodes::single(node.clone());
                if engine.has_validated(&node_object, *shape_idx) {
                    continue;
                }

                let inner_results = node_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                let is_valid = match inner_results {
                    Ok(results) => results.is_empty(),
                    Err(_) => false,
                };

                if !is_valid {
                    let msg = format!(
                        "Shape {}: Node({node_shape}) constraint not satisfied for {node}",
                        shape.id()
                    );
                    let vr = ValidationResult::new(fnode_obj.clone(), component_obj.clone(), shape.severity())
                        .with_path(maybe_path.cloned())
                        .with_message(MessageMap::from(msg))
                        .with_value(Some(node_object.clone()))
                        .with_source(Some(shape.id().clone()));
                    validation_results.push(vr.clone());
                    engine.record_validation(node_object, *shape_idx, vec![vr]);
                } else {
                    engine.record_validation(node_object, *shape_idx, Vec::new());
                }
            }
        }

        Ok(validation_results)
    }
}
