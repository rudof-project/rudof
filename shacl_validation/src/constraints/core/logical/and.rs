use std::ops::Not;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::shape::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use api::model::rdf::Rdf;
use shacl_ast::compiled::component::And;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use std::fmt::Debug;

impl<T: Triple> Validator<T> for And<R> {
    fn validate(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &R,
        engine: impl Engine<R>,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let and = |value_node: &S::Term| {
            self.shapes()
                .iter()
                .all(|shape| {
                    let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
                    match shape.validate(store, &engine, Some(&focus_nodes)) {
                        Ok(results) => results.is_empty(),
                        Err(_) => false,
                    }
                })
                .not()
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, and)
    }
}

impl<R: Rdf> NativeValidator<R> for And<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent<RS>,
        shape: &CompiledShape<R>,
        store: &R,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, NativeEngine, value_nodes)
    }
}

impl<S: Sparql> SparqlValidator<S> for And<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, SparqlEngine, value_nodes)
    }
}
