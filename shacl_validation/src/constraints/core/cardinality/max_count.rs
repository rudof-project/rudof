use api::model::rdf::Rdf;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MaxCount;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;
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
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;

impl<T: Triple> Validator<T> for MaxCount {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_count = |targets: &FocusNodes<S>| targets.len() > self.max_count();
        validate_with(component, shape, value_nodes, FocusNodeIteration, max_count)
    }
}

impl<R: Rdf> NativeValidator<R> for MaxCount {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &R,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, NativeEngine, value_nodes)
    }
}

impl<S: Sparql> SparqlValidator<S> for MaxCount {
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
