use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MaxLength;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Iri as _;
use srdf::Literal as _;
use srdf::Query;
use srdf::Sparql;
use srdf::Term;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for MaxLength {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_length = |value_node: &Q::Term| {
            if value_node.is_blank_node() {
                true
            } else if value_node.is_iri() {
                let iri: Q::IRI = match value_node.clone().try_into() {
                    Ok(iri) => iri,
                    Err(_) => todo!(),
                };
                iri.as_str().len() > self.max_length() as usize
            } else if value_node.is_literal() {
                let literal: Q::Literal = match value_node.clone().try_into() {
                    Ok(literal) => literal,
                    Err(_) => todo!(),
                };
                literal.lexical_form().len() > self.max_length() as usize
            } else {
                todo!()
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

impl<S: Sparql + Query> SparqlValidator<S> for MaxLength {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
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
