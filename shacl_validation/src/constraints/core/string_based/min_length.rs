use indoc::formatdoc;
use shacl_ast::compiled::component::MinLength;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::value_nodes::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for MinLength {
    fn validate_native<'a>(
        &self,
        _: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                if S::term_is_bnode(value_node) {
                    let result = ValidationResult::new(focus_node, Some(value_node));
                    Some(result)
                } else {
                    let string_representation = match S::term_as_string(value_node) {
                        Some(string_representation) => string_representation,
                        None => S::iri2iri_s(&S::term_as_iri(value_node).unwrap()).to_string(),
                    };
                    if string_representation.len() < self.min_length() as usize {
                        let result = ValidationResult::new(focus_node, Some(value_node));
                        Some(result)
                    } else {
                        None
                    }
                }
            });

        Ok(ValidationResults::new(results))
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MinLength {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .filter_map(move |(focus_node, value_node)| {
                if S::term_is_bnode(value_node) {
                    Some(ValidationResult::new(focus_node, Some(value_node)))
                } else {
                    let query = formatdoc! {
                        " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                        value_node, self.min_length()
                    };

                    let ask = match store.query_ask(&query) {
                        Ok(ask) => ask,
                        Err(_) => return None,
                    };

                    if !ask {
                        Some(ValidationResult::new(focus_node, Some(value_node)))
                    } else {
                        None
                    }
                }
            });

        Ok(ValidationResults::new(results))
    }
}
