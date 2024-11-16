use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::In;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::{NativeValidator, Validator};
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<T: Triple> Validator<T> for In<S> {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &Store<S>,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let r#in = |value_node: &S::Term| !self.values().contains(value_node);
        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            r#in,
            subsetting,
        )
    }
}

impl<R: Rdf> NativeValidator<R> for In<R> {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            subsetting,
        )
    }
}

impl<S: Sparql> SparqlValidator<S> for In<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            subsetting,
        )
    }
}
