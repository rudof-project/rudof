use indoc::formatdoc;
use shacl_ast::compiled::component::Pattern;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::helpers::validate_ask_with;
use crate::constraints::helpers::validate_with;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::native::NativeEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for Pattern {
    fn validate_native<'a>(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let language_in = |value_node: &S::Term| {
            if S::term_is_bnode(value_node) {
                true
            } else {
                todo!()
            }
        };
        validate_with(
            store,
            &NativeEngine,
            value_nodes,
            &ValueNodeIteration,
            language_in,
        )
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Pattern {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
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

        validate_ask_with(store, value_nodes, query)
    }
}
