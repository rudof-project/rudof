use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::LessThanOrEquals;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::SHACLPath;
use srdf::Sparql;
use std::fmt::Debug;

impl<S: Query + Debug + 'static> NativeValidator<S> for LessThanOrEquals<S> {
    fn validate_native(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
        _maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented(
            "LessThanOrEquals".to_string(),
        ))
    }
}

impl<S: Sparql + Debug + 'static> SparqlValidator<S> for LessThanOrEquals<S> {
    fn validate_sparql(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
        _maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented(
            "LessThanOrEquals".to_string(),
        ))
    }
}
