use crate::ir::components::And;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{ConstraintError, Validator, get_shape_from_idx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for And {
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
        let componet_obj = Object::iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let mut conforms = true;

                for idx in self.shapes().iter() {
                    let and_shape = get_shape_from_idx(shapes_graph, idx)?;
                    let inner_results =
                        and_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                    match inner_results {
                        Ok(results) => {
                            if !results.is_empty() {
                                conforms = false;
                                break;
                            }
                        },
                        Err(_) => {
                            conforms = false;
                            break;
                        },
                    }
                }

                if !conforms {
                    let node_obj = S::term_as_object(node).ok();

                    let vr = ValidationResult::new(fnode_obj.clone(), componet_obj.clone(), shape.severity())
                        .with_source(Some(shape.id().clone()))
                        .with_path(maybe_path.cloned())
                        .with_value(node_obj);
                    validation_results.push(vr);
                }
            }
        }

        Ok(validation_results)
    }
}
