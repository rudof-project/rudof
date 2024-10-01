use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::context::Context;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

use shacl_ast::compiled::component::MinCount;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

impl<S: SRDFBasic + 'static> Validator<S> for MinCount {
    fn validate(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        if self.min_count() == 0 {
            // If min_count is 0, then it always passes
            return Ok(ValidationResults::default());
        }

        let results = value_nodes
            .iter_focus_nodes()
            .filter_map(|(focus_node, value_nodes)| {
                if value_nodes.0.len() < self.min_count() {
                    Some(ValidationResult::new(focus_node, &evaluation_context, None))
                } else {
                    None
                }
            });

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for MinCount {
    fn validate_native(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MinCount {
    fn validate_sparql(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(evaluation_context, value_nodes)
    }
}
