use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Pattern;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;
use srdf::Term;

use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query> Validator<Q, NativeEngine> for Pattern {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let pattern = |value_node: &Q::Term| {
            if value_node.is_blank_node() {
                Ok(true)
            } else {
                Err(ValidateError::NotImplemented("Pattern"))
            }
        };
        validate_with(component, shape, value_nodes, ValueNodeIteration, pattern)
    }
}

impl<S: Sparql + Query> Validator<S, SparqlEngine> for Pattern {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let flags = self.flags().clone();
        let pattern = self.pattern().clone();

        let query = |value_node: &S::Term| match &flags {
            Some(flags) => formatdoc! {
                "ASK {{ FILTER (regex(str({}), {}, {})) }}",
                value_node, pattern, flags
            },
            None => formatdoc! {
                "ASK {{ FILTER (regex(str({}), {})) }}",
                value_node, pattern
            },
        };

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
