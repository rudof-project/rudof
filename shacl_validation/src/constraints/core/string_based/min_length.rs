use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinLength;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for MinLength {
    fn validate_native<'a>(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let min_length = |value_node: &S::Term| {
            if S::term_is_bnode(value_node) {
                true
            } else {
                let string_representation = match S::term_as_string(value_node) {
                    Some(string_representation) => string_representation,
                    None => S::iri2iri_s(&S::term_as_iri(value_node).unwrap()).to_string(),
                };
                string_representation.len() < self.min_length() as usize
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

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MinLength {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
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
