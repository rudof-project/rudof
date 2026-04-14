use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use crate::ir::components::Not;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{get_shape_from_idx, ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::error::ValidationError;
use crate::validation::focus_nodes::FocusNodes;
use crate::validation::report::ValidationResult;
use crate::validation::validator::Validate;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Not {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();

        for (fnode, nodes) in value_nodes.iter() {
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let not_shape = get_shape_from_idx(shapes_graph, self.shape())?;
                let inner_results = not_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                let is_valid_inside = match inner_results {
                    Ok(results) => results.is_empty(),
                    Err(_) => false, // TODO - Should we fail instead of considering it valid?
                };
                if is_valid_inside {
                    let msg = format!("Shape: {}. NOT constraint not satisfied for focus node {fnode} and internal shape {}", shape.id(), not_shape.id());
                    let component = Object::iri(component.into());
                    let node_object = S::term_as_object(node)?;
                    validation_results.push(
                        ValidationResult::new(node_object, component.clone(), shape.severity())
                            .with_message(Some(msg))
                            .with_path(maybe_path.cloned()),
                    );
                }
            }
        }

        Ok(validation_results)
    }
}
