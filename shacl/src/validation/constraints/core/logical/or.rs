use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use crate::ir::components::Or;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{get_shape_from_idx, ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::error::ValidationError;
use crate::validation::focus_nodes::FocusNodes;
use crate::validation::report::ValidationResult;
use crate::validation::validator::Validate;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Or {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();

        for (_, nodes) in value_nodes.iter() {
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
                    let msg = "OR not satisfied".to_string();
                    let component = Object::iri(component.into());
                    validation_results.push(
                        ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                            .with_message(Some(msg))
                            .with_path(maybe_path.cloned())
                    );
                }
            }
        }

        Ok(validation_results)
    }
}
