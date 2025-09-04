use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::Xone;
use shacl_ir::compiled::shape::ShapeIR;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::fmt::Debug;

use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::engine::Engine;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shape_validation::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Xone {
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
        let xone = |value_node: &S::Term| {
            self.shapes()
                .iter()
                .filter(|shape| {
                    let focus_nodes = FocusNodes::from_iter(std::iter::once(value_node.clone()));
                    match shape.validate(store, &engine, Some(&focus_nodes), Some(shape)) {
                        Ok(results) => results.is_empty(),
                        Err(_) => false,
                    }
                })
                .count()
                .ne(&1usize)
        };

        let message = "Xone not satisfied".to_string();
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            xone,
            &message,
            maybe_path,
        )
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Xone {
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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Xone {
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
