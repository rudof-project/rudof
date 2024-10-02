use shacl_ast::compiled::component::LessThan;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for LessThan<S> {
    fn validate_native(
        &self,
        store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for LessThan<S> {
    fn validate_sparql(
        &self,
        store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}
