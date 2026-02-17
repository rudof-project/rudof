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
use rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF, term::Object};
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::Not;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use std::fmt::Debug;
use tracing::debug;
use tracing::info;
use tracing::trace;

impl<S: NeighsRDF + Debug> Validator<S> for Not {
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
        let mut validation_results = Vec::new();

        for (focus_node, nodes) in value_nodes.iter() {
            debug!(
                "Validating NOT constraint for shape {} and node: {focus_node}",
                shape.id()
            );
            let all_nodes = nodes.iter().cloned().collect::<Vec<_>>();
            info!(
                "Before loop, focus node: {focus_node}, all nodes: {}",
                all_nodes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")
            );
            for node in nodes.iter() {
                info!("Validating NOT constraint for node: {node}");
                let focus_nodes = FocusNodes::from_iter(std::iter::once(node.clone()));
                let not_shape = get_shape_from_idx(shapes_graph, self.shape())?;
                debug!("Validating NOT constraint with internal shape {}", not_shape.id());
                let inner_results = not_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                let is_valid_inside = match inner_results {
                    Err(results) => {
                        trace!("Internal shape of NOT constraint failed {:?}", results);
                        // TODO: Should we fail instead of considering it valid?
                        false
                    },
                    Ok(results) if results.is_empty() => true,
                    Ok(results) => {
                        trace!("Internal shape of NOT constraint failed with violations: {:?}", results);
                        false
                    },
                };
                info!("NOT constraint validation result for node {node}, is_valid_inside?={is_valid_inside}");
                if is_valid_inside {
                    let message = format!(
                        "Shape: {}. NOT constraint not satisfied for focus node {} and internal shape {}",
                        shape.id(),
                        focus_node,
                        not_shape.id()
                    );
                    let component = Object::iri(component.into());
                    let node_object = S::term_as_object(node)?;
                    validation_results.push(
                        ValidationResult::new(node_object, component.clone(), shape.severity())
                            .with_message(message.as_str())
                            .with_path(maybe_path.clone()),
                    );
                }
            }
        }
        Ok(validation_results)
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Not {
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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Not {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            &mut SparqlEngine::new(),
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}
