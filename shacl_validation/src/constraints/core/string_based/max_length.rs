use indoc::formatdoc;
use shacl_ast::compiled::component::MaxLength;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::helpers::validate_ask_with;
use crate::constraints::helpers::validate_with;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for MaxLength {
    fn validate_native<'a>(
        &self,
        _: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let max_length = |value_node: &S::Term| {
            if S::term_is_bnode(value_node) {
                true
            } else {
                let string_representation = match S::term_as_string(value_node) {
                    Some(string_representation) => string_representation,
                    None => S::iri2iri_s(&S::term_as_iri(value_node).unwrap()).to_string(),
                };
                string_representation.len() > self.max_length() as usize
            }
        };

        validate_with(value_nodes, &ValueNodeIteration, max_length)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MaxLength {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let max_length_value = self.max_length();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) <= {}) }} ",
                value_node, max_length_value
            }
        };

        validate_ask_with(store, value_nodes, query)
    }
}
