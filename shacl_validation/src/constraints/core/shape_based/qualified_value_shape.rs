use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::focus_nodes::FocusNodes;
use crate::shacl_engine::Engine;
use crate::shacl_engine::native::NativeEngine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::shape_validation::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::QualifiedValueShape;
use shacl_ir::compiled::shape::ShapeIR;
use srdf::NeighsRDF;
use srdf::Object;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::collections::HashSet;
use std::fmt::Debug;
use tracing::debug;

impl<S: NeighsRDF + Debug> Validator<S> for QualifiedValueShape {
    fn validate(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        engine: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
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
                let inner_results =
                    self.shape()
                        .validate(store, &engine, Some(&focus_nodes), Some(self.shape()));
                let mut is_valid = match inner_results {
                    Err(e) => {
                        debug!(
                            "Error validating node {node} with shape {}: {e}",
                            self.shape().id()
                        );
                        false
                    }
                    Ok(results) => {
                        if !results.is_empty() {
                            debug!(
                                "Node doesn't conform to shape {}, results: {}",
                                self.shape().id(),
                                results
                                    .iter()
                                    .map(|r| format!(" {r:?}"))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            );
                            false
                        } else {
                            debug!(
                                "Node {node} initially conforms to shape {}",
                                self.shape().id()
                            );
                            true
                        }
                    }
                };
                if !self.siblings().is_empty() && is_valid {
                    // If there are siblings, check that none of them validate
                    debug!("Checking siblings for node {node}: {:?}", self.siblings());
                    for sibling in self.siblings().iter() {
                        debug!("Checking {node} with sibling shape: {}", sibling.id());
                        let sibling_results = self.shape().validate(
                            store,
                            &engine,
                            Some(&focus_nodes),
                            Some(sibling),
                        );
                        let sibling_is_valid =
                            sibling_results.is_ok() && sibling_results.unwrap().is_empty();
                        debug!(
                            "Result of node {node} with sibling shape {}: {sibling_is_valid}",
                            sibling.id()
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
            if let Some(min_count) = self.qualified_min_count() {
                if valid_counter < min_count {
                    let message = format!(
                        "QualifiedValueShape: only {valid_counter} nodes conform to shape {}, which is less than minCount: {min_count}. Focus node: {focus_node}",
                        self.shape().id()
                    );
                    let validation_result = ValidationResult::new(
                        shape.id().clone(),
                        component.clone(),
                        shape.severity(),
                    )
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                    validation_results.insert(validation_result);
                }
            }
            if let Some(max_count) = self.qualified_max_count() {
                if valid_counter > max_count {
                    let message = format!(
                        "QualifiedValueShape: {valid_counter} nodes conform to shape {}, which is greater than maxCount: {max_count}. Focus node: {focus_node}",
                        self.shape().id()
                    );
                    let validation_result = ValidationResult::new(
                        shape.id().clone(),
                        component.clone(),
                        shape.severity(),
                    )
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                    validation_results.insert(validation_result);
                }
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
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            source_shape,
            maybe_path,
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
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}
