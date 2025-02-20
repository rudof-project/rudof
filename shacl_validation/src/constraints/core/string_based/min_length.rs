use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinLength;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Iri as _;
use srdf::Literal as _;
use srdf::Query;
use srdf::Sparql;
use srdf::Term;
use std::fmt::Debug;

impl<S: Query + Debug + 'static> NativeValidator<S> for MinLength {
    fn validate_native<'a>(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_length = |value_node: &S::Term| {
            if S::term_is_bnode(value_node) {
                true
            } else if value_node.is_iri() {
                let iri: S::IRI = match value_node.clone().try_into() {
                    Ok(iri) => iri,
                    Err(_) => todo!(),
                };
                iri.as_str().len() > self.min_length() as usize
            } else if S::term_is_literal(value_node) {
                let literal: S::Literal = match value_node.clone().try_into() {
                    Ok(literal) => literal,
                    Err(_) => todo!(),
                };
                literal.as_str().len() > self.min_length() as usize
            } else {
                todo!()
            }
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            min_length,
        )
    }
}

impl<S: Sparql + Debug + 'static> SparqlValidator<S> for MinLength {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_length_value = self.min_length();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                value_node, min_length_value
            }
        };

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
