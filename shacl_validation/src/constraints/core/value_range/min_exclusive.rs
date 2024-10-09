use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinExclusive;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for MinExclusive<S> {
    fn validate_native(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        Err(ConstraintError::NotImplemented("MinExclusive".to_string()))
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MinExclusive<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let min_exclusive_value = self.min_exclusive().clone();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} < {}) }} ",
                value_node, min_exclusive_value
            }
        };

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
