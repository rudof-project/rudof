use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::get_shape_from_idx;
use crate::focus_nodes::FocusNodes;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::shape_validation::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF, term::Object};
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::QualifiedValueShape;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use std::collections::HashSet;
use std::fmt::Debug;
use tracing::trace;

impl<S: NeighsRDF + Debug> Validator<S> for QualifiedValueShape {
    fn validate(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        // TODO: It works but it returns duplicated validation results
        // I tried to use a HashSet but it still doesn't remove duplicates...
        let mut validation_results = HashSet::new();
        let component = Object::iri(component.into());

        for (focus_node, nodes) in value_nodes.iter() {
            let mut valid_counter = 0;
            // Count how many nodes conform to the shape
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::from_iter(std::iter::once(node.clone()));
                let shape = get_shape_from_idx(shapes_graph, self.shape())?;
                let inner_results = shape.validate(store, engine, Some(&focus_nodes), Some(&shape), shapes_graph);
                let mut is_valid = match inner_results {
                    Err(e) => {
                        trace!("Error validating node {node} with shape {}: {e}", shape.id());
                        false
                    },
                    Ok(results) => {
                        if !results.is_empty() {
                            trace!(
                                "Node doesn't conform to shape {}, results: {}",
                                shape.id(),
                                results
                                    .iter()
                                    .map(|r| format!(" {r:?}"))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            );
                            false
                        } else {
                            trace!("Node {node} initially conforms to shape {}", shape.id());
                            true
                        }
                    },
                };
                if !self.siblings().is_empty() && is_valid {
                    // If there are siblings, check that none of them validate
                    trace!("Checking siblings for node {node}: {:?}", self.siblings());
                    for sibling in self.siblings().iter() {
                        let sibling_shape = get_shape_from_idx(shapes_graph, sibling)?;
                        trace!("Checking {node} with sibling shape: {}", sibling_shape.id());
                        let sibling_results = sibling_shape.validate(
                            store,
                            engine,
                            Some(&focus_nodes),
                            Some(&sibling_shape),
                            shapes_graph,
                        );
                        let sibling_is_valid = sibling_results.is_ok() && sibling_results.unwrap().is_empty();
                        trace!(
                            "Result of node {node} with sibling shape {}: {sibling_is_valid}",
                            sibling_shape.id()
                        );
                        if sibling_is_valid {
                            is_valid = false;
                            break;
                        }
                    }
                }
                if is_valid {
                    valid_counter += 1
                }
            }
            if let Some(min_count) = self.qualified_min_count()
                && valid_counter < min_count
            {
                let message = format!(
                    "QualifiedValueShape: only {valid_counter} nodes conform to shape {}, which is less than minCount: {min_count}. Focus node: {focus_node}",
                    shape.id()
                );
                let validation_result = ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                validation_results.insert(validation_result);
            }
            if let Some(max_count) = self.qualified_max_count()
                && valid_counter > max_count
            {
                let message = format!(
                    "QualifiedValueShape: {valid_counter} nodes conform to shape {}, which is greater than maxCount: {max_count}. Focus node: {focus_node}",
                    shape.id()
                );
                let validation_result = ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                validation_results.insert(validation_result);
            }
        }
        Ok(validation_results.iter().cloned().collect())
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for QualifiedValueShape {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            engine,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for QualifiedValueShape {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shape_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            &mut SparqlEngine::new(),
            value_nodes,
            source_shape,
            maybe_path,
            shape_graph,
        )
    }
}
