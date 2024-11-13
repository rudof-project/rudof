use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::LessThanOrEquals;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDF;
use std::fmt::Debug;

impl<R: Rdf> NativeValidator<R> for LessThanOrEquals<S> {
    fn validate_native(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented(
            "LessThanOrEquals".to_string(),
        ))
    }
}

impl<S: Sparql> SparqlValidator<S> for LessThanOrEquals<S> {
    fn validate_sparql(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented(
            "LessThanOrEquals".to_string(),
        ))
    }
}
