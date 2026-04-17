use std::collections::HashSet;
use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use crate::ir::components::QualifiedValueShape;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{get_shape_from_idx, ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validator::engine::{Engine, SparqlEngine, Validate};
use crate::validator::error::ValidationError;
use crate::validator::report::ValidationResult;
use crate::validator::nodes::{ValueNodes, FocusNodes};

impl<S: NeighsRDF + Debug> Validator<S> for QualifiedValueShape {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        // TODO - It works but it returns duplicated validation results
        // I tried to use a HashSet but it still doesn't remove duplicates...
        let mut validation_results = HashSet::new();
        let component = Object::iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let mut valid_counter = 0;
            // Count how many nodes conform to the shape
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let shape = get_shape_from_idx(shapes_graph, self.shape())?;
                let inner_results = shape.validate(store, engine, Some(&focus_nodes), Some(&shape), shapes_graph);
                let mut is_valid = match inner_results {
                    Ok(results) => results.is_empty(),
                    Err(_) => false,
                };

                if !self.siblings().is_empty() && is_valid {
                    // If there are siblings, check that none of them validate
                    for sibling in self.siblings().iter() {
                        let sibling_shape = get_shape_from_idx(shapes_graph, sibling)?;
                        let sibling_results = sibling_shape.validate(
                            store,
                            engine,
                            Some(&focus_nodes),
                            Some(&sibling_shape),
                            shapes_graph,
                        );
                        let sibling_is_valid = sibling_results.is_ok() && sibling_results.unwrap().is_empty();
                        if sibling_is_valid {
                            is_valid = false;
                            break;
                        }

                    }
                }

                if is_valid { valid_counter += 1; }
            }

            if let Some(min_count) = self.qualified_min_count() && valid_counter < min_count {
                    let msg = format!("QualifiedValueShape: only {valid_counter} nodes conform to shape {}, which is less than minCount: {min_count}. Focus node: {fnode}", shape.id());
                let validation_result = ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                    .with_message(Some(msg))
                    .with_path(maybe_path.cloned());
                validation_results.insert(validation_result);
            }

            if let Some(max_count) = self.qualified_max_count() && valid_counter > max_count {
                    let msg = format!("QualifiedValueShape: {valid_counter} nodes conform to shape {}, which is grater than maxCount: {max_count}. Focus node: {fnode}", shape.id());
                let validation_result = ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                    .with_path(maybe_path.cloned())
                    .with_message(Some(msg));
                validation_results.insert(validation_result);
            }
        }

        Ok(validation_results.into_iter().collect())
    }
}
