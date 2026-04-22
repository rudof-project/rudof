use crate::ir::components::Not;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::{ConstraintError, Validator, get_shape_from_idx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Not {
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

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let not_shape = get_shape_from_idx(shapes_graph, self.shape())?;
                let inner_results = not_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                let is_valid_inside = match inner_results {
                    Ok(results) => results.is_empty(),
                    Err(_) => false, // TODO - Should we fail instead of considering it valid?
                };
                if is_valid_inside {
                    let msg = format!(
                        "Shape: {}. NOT constraint not satisfied for focus node {fnode} and internal shape {}",
                        shape.id(),
                        not_shape.id()
                    );
                    let component = Object::iri(component.into());
                    let node_object = S::term_as_object(node).ok();
                    let vr = ValidationResult::new(fnode_obj.clone(), component.clone(), shape.severity())
                        .with_message(MessageMap::from(msg))
                        .with_path(maybe_path.cloned())
                        .with_source(Some(shape.id().clone()))
                        .with_value(node_object);
                    validation_results.push(vr);
                }
            }
        }

        Ok(validation_results)
    }
}
