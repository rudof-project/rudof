use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MaxInclusive;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDF;
use std::fmt::Debug;

impl<R: Rdf> NativeValidator<R> for MaxInclusive<S> {
    fn validate_native(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("MaxInclusive".to_string()))
    }
}

impl<S: Sparql> SparqlValidator<S> for MaxInclusive<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_inclusive_value = self.max_inclusive().clone();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} >= {}) }} ",
                value_node, max_inclusive_value
            }
        };

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
