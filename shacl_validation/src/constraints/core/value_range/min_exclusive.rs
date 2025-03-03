use std::fmt::Debug;

use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinExclusive;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_ask_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<Q: Query + Debug + 'static, E: Engine<Q>> Validator<Q, E> for MinExclusive<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
        engine: E,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("MinExclusive".to_string()))
    }
}

impl<S: Sparql + Query + Debug + 'static> SparqlValidator<S> for MinExclusive<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
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
