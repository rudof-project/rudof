use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use crate::ir::components::Node;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{get_shape_from_idx, ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validator::engine::{Engine, SparqlEngine, Validate};
use crate::validator::error::ValidationError;
use crate::validator::report::ValidationResult;
use crate::validator::nodes::{FocusNodes, ValueNodes};

impl<S: NeighsRDF + Debug> Validator<S> for Node {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();
        let shape_idx = self.shape();
        let node_shape = get_shape_from_idx(shapes_graph, shape_idx)?;

        for (_, nodes) in value_nodes.iter() {
            for node in nodes.iter() {
                let node_object = S::term_as_object(node)?;
                let focus_nodes = FocusNodes::single(node.clone());
                if engine.has_validated(&node_object, *shape_idx) { continue; }

                let inner_results = node_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                let is_valid = match inner_results {
                    Ok(results) => results.is_empty(),
                    Err(_) => false,
                };

                if !is_valid {
                        let msg = format!("Shape {}: Node({node_shape}) constraint not satisfied for {node}", shape.id());
                    let validation_result = ValidationResult::new(node_object.clone(), Object::iri(component.into()), shape.severity())
                        .with_path(maybe_path.cloned())
                        .with_message(Some(msg));
                    validation_results.push(validation_result.clone());
                    engine.record_validation(node_object, *shape_idx, vec![validation_result]);
                } else {
                    engine.record_validation(node_object, *shape_idx, Vec::new());
                }
            }
        }

        Ok(validation_results)
    }
}
