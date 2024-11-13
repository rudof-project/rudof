use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;

use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinCount;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;
use std::fmt::Debug;

impl<T: Triple> Validator<T> for MinCount {
    fn validate(
        &self,
        component: &CompiledComponent<T>,
        shape: &CompiledShape<T>,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        if self.min_count() == 0 {
            // If min_count is 0, then it always passes
            return Ok(Default::default());
        }
        let min_count = |targets: &FocusNodes<S>| targets.len() < self.min_count();
        validate_with(component, shape, value_nodes, FocusNodeIteration, min_count)
    }
}

impl<R: Rdf> NativeValidator<R> for MinCount {
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

impl<S: Sparql> SparqlValidator<S> for MinCount {
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
