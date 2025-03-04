use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinInclusive;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_ask_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for MinInclusive<Q> {
    fn validate(
        &self,
        _component: &CompiledComponent<Q>,
        _shape: &CompiledShape<Q>,
        _store: &Q,
        _value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        Err(ValidateError::NotImplemented("MinInclusive"))
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for MinInclusive<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let min_inclusive_value = self.min_inclusive().clone();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} <= {}) }} ",
                value_node, min_inclusive_value
            }
        };

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
