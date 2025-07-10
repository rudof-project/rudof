use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MaxLength;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Iri as _;
use srdf::Literal as _;
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

impl<Q: Query> Validator<Q, NativeEngine> for MaxLength {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let max_length = |value_node: &Q::Term| {
            if value_node.is_blank_node() {
                Ok(true)
            } else {
                let string_length = value_node
                    .clone()
                    .try_into()
                    .map(|iri: Q::IRI| iri.as_str().len())
                    .or_else(|_| {
                        value_node
                            .clone()
                            .try_into()
                            .map(|literal: Q::Literal| literal.lexical_form().len())
                    })
                    .unwrap_or_else(|_| unreachable!());

                Ok(string_length > self.max_length() as usize)
            }
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            max_length,
        )
    }
}

impl<S: Sparql + Query> Validator<S, SparqlEngine> for MaxLength {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let max_length_value = self.max_length();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) <= {}) }} ",
                value_node, max_length_value
            }
        };

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
