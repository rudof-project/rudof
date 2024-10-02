use shacl_ast::compiled::component::MaxCount;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::runner::native::NativeValidatorRunner;
use crate::runner::sparql::SparqlValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

impl<S: SRDFBasic + 'static> Validator<S> for MaxCount {
    fn validate(
        &self,
        _: &S,
        _: impl ValidatorRunner<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_focus_nodes()
            .filter_map(|(focus_node, value_nodes)| {
                if value_nodes.0.len() > self.max_count() {
                    Some(ValidationResult::new(focus_node, None))
                } else {
                    None
                }
            });

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for MaxCount {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, NativeValidatorRunner, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MaxCount {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, SparqlValidatorRunner, value_nodes)
    }
}
