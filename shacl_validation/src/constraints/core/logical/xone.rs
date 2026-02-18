use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF, term::Object};
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::Xone;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use std::fmt::Debug;

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

impl<S: NeighsRDF + Debug> Validator<S> for Xone {
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
        for (_focus_node, nodes) in value_nodes.iter() {
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::from_iter(std::iter::once(node.clone()));
                let mut conforming_shapes = 0;
                for shape_idx in self.shapes().iter() {
                    let internal_shape = get_shape_from_idx(shapes_graph, shape_idx)?;
                    let inner_results =
                        internal_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                    match inner_results {
                        Err(e) => {
                            tracing::trace!("Error validating node {node} with shape {}: {e}", internal_shape.id());
                        },
                        Ok(results) => {
                            if results.is_empty() {
                                conforming_shapes += 1;
                            }
                        },
                    }
                }
                if conforming_shapes != 1 {
                    let message = format!(
                        "Shape {}: Xone constraint not satisfied for node {}. Number of conforming shapes: {}",
                        shape.id(),
                        node,
                        conforming_shapes
                    );
                    let component = Object::iri(component.into());
                    validation_results.push(
                        ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                            .with_message(message.as_str())
                            .with_path(maybe_path.clone()),
                    );
                }
            }
        }
        Ok(validation_results)
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Xone {
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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Xone {
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
