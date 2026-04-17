use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use crate::error::ConstraintError;
use crate::ir::components::Xone;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{get_shape_from_idx, Validator};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;

impl<S: NeighsRDF + Debug> Validator<S> for Xone {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();

        for (_, nodes) in value_nodes.iter() {
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let mut conforming_shapes = 0;
                for idx in self.shapes().iter() {
                    let internal_shape = get_shape_from_idx(shapes_graph, idx)?;
                    let inner_results = internal_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                    match inner_results {
                        Ok(results) => if results.is_empty() {
                          conforming_shapes += 1;
                        },
                        Err(_) => {}
                    }
                }
                if conforming_shapes != 1 {
                    let msg = format!("Shape {}: Xone constraint not satisfied for node {node}. Number of conforming shapes: {conforming_shapes}", shape.id());
                    let component = Object::iri(component.into());
                    validation_results.push(
                        ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                            .with_message(Some(msg))
                            .with_path(maybe_path.cloned()),
                    )
                }
            }
        }

        Ok(validation_results)
    }
}
