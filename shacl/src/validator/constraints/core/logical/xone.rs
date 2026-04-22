use crate::error::ConstraintError;
use crate::ir::components::Xone;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::{Validator, get_shape_from_idx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Xone {
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
        let component = Object::iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let mut conforming_shapes = 0;
                for idx in self.shapes().iter() {
                    let internal_shape = get_shape_from_idx(shapes_graph, idx)?;
                    let inner_results =
                        internal_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                    if let Ok(results) = inner_results && results.is_empty() {
                        conforming_shapes += 1;
                    }
                }
                if conforming_shapes != 1 {
                    let node_obj = S::term_as_object(node).ok();
                    let msg = format!(
                        "Shape {}: Xone constraint not satisfied for node {node}. Number of conforming shapes: {conforming_shapes}",
                        shape.id()
                    );
                    let vr = ValidationResult::new(fnode_obj.clone(), component.clone(), shape.severity())
                        .with_message(MessageMap::from(msg))
                        .with_path(maybe_path.cloned())
                        .with_value(node_obj)
                        .with_source(Some(shape.id().clone()));
                    validation_results.push(vr);
                }
            }
        }

        Ok(validation_results)
    }
}
