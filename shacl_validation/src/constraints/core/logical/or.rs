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
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::Or;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use rdf::rdf_core::{NeighsRDF, query::QueryRDF, SHACLPath, term::Object};
use std::fmt::Debug;
use tracing::debug;

impl<S: NeighsRDF + Debug> Validator<S> for Or {
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
                let mut conforms = false;
                for shape_idx in self.shapes().iter() {
                    let or_shape = get_shape_from_idx(shapes_graph, shape_idx)?;
                    let inner_results = or_shape.validate(
                        store,
                        engine,
                        Some(&focus_nodes),
                        Some(shape),
                        shapes_graph,
                    );
                    match inner_results {
                        Err(err) => {
                            debug!("Or: Error validating {node} with shape {shape}: {err}");
                            conforms = true;
                        }
                        Ok(results) => {
                            if results.is_empty() {
                                conforms = true;
                                break;
                            }
                        }
                    }
                }
                if !conforms {
                    let message = "OR not satisfied".to_string();
                    let component = Object::iri(component.into());
                    validation_results.push(
                        ValidationResult::new(
                            shape.id().clone(),
                            component.clone(),
                            shape.severity(),
                        )
                        .with_message(message.as_str())
                        .with_path(maybe_path.clone()),
                    );
                }
            }
        }
        Ok(validation_results)
        /*let or = |value_node: &S::Term| {
            self.shapes()
                .iter()
                .any(|shape_idx| {
                    let shape = shapes_graph.get_shape_from_idx(shape_idx).expect(
                        format!("Internal error: Shape {} not found in shapes graph", shape)
                            .as_str(),
                    );
                    match shape.validate(
                        store,
                        &engine,
                        Some(&FocusNodes::from_iter(std::iter::once(value_node.clone()))),
                        Some(shape),
                        shapes_graph,
                    ) {
                        Ok(validation_results) => validation_results.is_empty(),
                        Err(err) => {
                            debug!("Or: Error validating {value_node} with shape {shape}: {err}");
                            true
                        }
                    }
                })
                .not()
        };

        let message = "OR not satisfied".to_string();
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            or,
            &message,
            maybe_path,
        )*/
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Or {
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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Or {
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
