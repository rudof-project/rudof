use indoc::formatdoc;
use shacl_ast::compiled::component::MaxInclusive;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::value_nodes::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for MaxInclusive<S> {
    fn validate_native(
        &self,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MaxInclusive<S> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .filter_map(move |(focus_node, value_node)| {
                let query = formatdoc! {
                    " ASK {{ FILTER ({} > {}) }} ",
                    value_node, self.max_inclusive()
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
            });

        Ok(ValidationResults::new(results))
    }
}
