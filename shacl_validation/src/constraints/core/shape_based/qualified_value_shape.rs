use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::QualifiedValueShape;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Rdf;

use crate::constraints::Validator;
use crate::engine::Engine;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<R: Rdf, E: Engine<R>> Validator<R, E> for QualifiedValueShape<R> {
    fn validate(
        &self,
        _component: &CompiledComponent<R>,
        _shape: &CompiledShape<R>,
        _store: &R,
        _value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        Err(ValidateError::NotImplemented("QualifiedValueShape"))
    }
}
