use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MaxExclusive;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::Query;
use std::fmt::Debug;

impl<S: Query + Debug + 'static> NativeValidator<S> for MaxExclusive<S> {
    fn validate_native(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("MaxExclusive".to_string()))
    }
}

impl<S: QuerySRDF + Debug + 'static> SparqlValidator<S> for MaxExclusive<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_exclusive_value = self.max_exclusive().clone();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} > {}) }} ",
                value_node, max_exclusive_value
            }
        };

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
