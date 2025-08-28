use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::Xone;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::fmt::Debug;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::shape_validation::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Xone {
    fn validate(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        engine: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let xone = |value_node: &S::Term| {
            self.shapes()
                .iter()
                .filter(|shape| {
                    let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
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
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
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
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
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
