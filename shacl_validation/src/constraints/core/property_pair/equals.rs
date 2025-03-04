use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Equals;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for Equals<Q> {
    fn validate(
        &self,
        _component: &CompiledComponent<Q>,
        _shape: &CompiledShape<Q>,
        _store: &Q,
        _value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        Err(ValidateError::NotImplemented("Equals"))
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for Equals<S> {}
