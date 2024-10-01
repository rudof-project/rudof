use indoc::formatdoc;
use shacl_ast::compiled::component::Pattern;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::context::Context;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for Pattern {
    fn validate_native<'a>(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                if S::term_is_bnode(value_node) {
                    let result =
                        ValidationResult::new(focus_node, &evaluation_context, Some(value_node));
                    Some(result)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Pattern {
    fn validate_sparql(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .filter_map(move |(focus_node, value_node)| {
                if S::term_is_bnode(value_node) {
                    Some(ValidationResult::new(
                        focus_node,
                        &evaluation_context,
                        Some(value_node),
                    ))
                } else {
                    let query = match self.flags() {
                        Some(flags) => formatdoc! {
                            "ASK {{ FILTER (regex(str({}), {}, {})) }}",
                            value_node, self.pattern(), flags
                        },
                        None => formatdoc! {
                            "ASK {{ FILTER (regex(str({}), {})) }}",
                            value_node, self.pattern()
                        },
                    };

                    let ask = match validation_context.store().query_ask(&query) {
                        Ok(ask) => ask,
                        Err(_) => return None,
                    };

                    if !ask {
                        Some(ValidationResult::new(
                            focus_node,
                            &evaluation_context,
                            Some(value_node),
                        ))
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}
