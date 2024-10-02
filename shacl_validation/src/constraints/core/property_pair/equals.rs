use shacl_ast::compiled::component::Equals;
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
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

impl<S: SRDFBasic + 'static> Validator<S> for Equals<S> {
    fn validate(
        &self,
        _store: &S,
        _runner: impl ValidatorRunner<S>,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for Equals<S> {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, NativeValidatorRunner, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Equals<S> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, SparqlValidatorRunner, value_nodes)
    }
}
