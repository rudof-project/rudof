use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use crate::ir::components::Or;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::{get_shape_from_idx, ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validator::engine::{Engine, SparqlEngine, Validate};
use crate::validator::nodes::FocusNodes;
use crate::validator::report::ValidationResult;
use crate::validator::nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Or {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();
        let component = Object::iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let mut conforms = false;
                for idx in self.shapes().iter() {
                    let or_shape = get_shape_from_idx(shapes_graph, idx)?;
                    let inner_results = or_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                    match inner_results {
                        Ok(results) => if results.is_empty() {
                            conforms = true;
                            break;
                        },
                        Err(_) => conforms = true,
                    }
                }
                if !conforms {
                    let node_obj = S::term_as_object(node).ok();
                    let msg = "OR not satisfied".to_string();
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
