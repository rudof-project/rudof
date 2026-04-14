use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::ir::components::And;
use crate::validation::constraints::{get_shape_from_idx, ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::error::ValidationError;
use crate::validation::focus_nodes::FocusNodes;
use crate::validation::report::ValidationResult;
use crate::validation::validator::Validate;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for And {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();

        for (_, nodes) in value_nodes.iter() {
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let mut all_conform = true;
                for idx in self.shapes().iter() {
                    let shape = get_shape_from_idx(shapes_graph, idx)?;
                    let inner_results = shape.validate(store, engine, Some(&focus_nodes), Some(&shape), shapes_graph);
                    match inner_results {
                        Ok(results) => if !results.is_empty() {
                            all_conform = false;
                            validation_results.extend(results);
                        },
                        Err(_) => all_conform = false,
                    }
                    if !all_conform { break; }
                }
            }
        }

        Ok(validation_results)
    }
}
