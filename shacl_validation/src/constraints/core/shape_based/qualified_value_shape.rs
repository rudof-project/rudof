use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
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
use std::fmt::Debug;

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
        let mut validation_results = Vec::new();
        let component = Object::iri(component.into());
        let severity = Object::iri(shape.severity().iri());

        for (focus_node, nodes) in value_nodes.iter() {
            let mut valid_counter = 0;
            // Cound how many nodes conform to the shape
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::from_iter(std::iter::once(node.clone()));
                let inner_results =
                    self.shape()
                        .validate(store, &engine, Some(&focus_nodes), Some(self.shape()));
                let is_valid = !inner_results.is_err() && inner_results.unwrap().is_empty();
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
                        severity.clone(),
                    )
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                    validation_results.push(validation_result);
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
                        severity.clone(),
                    )
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                    validation_results.push(validation_result);
                }
            }
        }
        Ok(validation_results)
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
