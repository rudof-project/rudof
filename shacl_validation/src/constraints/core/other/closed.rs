use shacl_ast::compiled::component::Closed;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::Validator;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::context::Context;
use crate::context::ValidationContext;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

impl<S: SRDFBasic + 'static> Validator<S> for Closed<S> {
    fn validate(
        &self,
        _validation_context: &ValidationContext<S>,
        _evaluation_context: Context<S>,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for Closed<S> {
    fn validate_native(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Closed<S> {
    fn validate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(evaluation_context, value_nodes)
    }
}
