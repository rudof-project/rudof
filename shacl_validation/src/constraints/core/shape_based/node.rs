use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::focus_nodes::FocusNodes;
use crate::shacl_engine::Engine;
use crate::shacl_engine::engine;
use crate::shacl_engine::native::NativeEngine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::shape_validation::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::Node;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::fmt::Debug;
use tracing::trace;

impl<S: NeighsRDF + Debug> Validator<S> for Node {
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
        let shape_idx = self.shape();
        let node_shape = shapes_graph.get_shape_from_idx(shape_idx).expect(
            format!(
                "Internal error: Shape {} in Node constraint not found in shapes graph",
                self.shape()
            )
            .as_str(),
        );
        for (focus_node, nodes) in value_nodes.iter() {
            trace!(
                "Validating Node constraint for shape {} and node: {focus_node}",
                shape.id()
            );
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::from_iter(std::iter::once(node.clone()));
                let inner_results = node_shape.validate(
                    store,
                    engine,
                    Some(&focus_nodes),
                    Some(shape),
                    shapes_graph,
                );
                let is_valid = match inner_results {
                    Err(_) => false,
                    Ok(results) => results.is_empty(),
                };
                let node_object = S::term_as_object(node)?;
                if !is_valid {
                    let message = format!(
                        "Shape {}: Node({node_shape}) constraint not satisfied for {node}",
                        shape.id(),
                    );
                    let component = srdf::Object::iri(component.into());
                    let result = ValidationResult::new(
                        shape.id().clone(),
                        component.clone(),
                        shape.severity(),
                    )
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                    validation_results.push(result.clone());
                    engine.record_validation(node_object, *shape_idx, vec![result]);
                } else {
                    engine.record_validation(node_object, *shape_idx, Vec::new());
                }
            }
        }
        Ok(validation_results)
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Node {
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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Node {
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
