use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Disjoint;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for Disjoint<Q> {
    fn validate(
        &self,
        _component: &CompiledComponent<Q>,
        _shape: &CompiledShape<Q>,
        _store: &Q,
        _value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Disjoint".to_string()))
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for Disjoint<S> {
    fn validate_sparql(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Disjoint".to_string()))
    }
}
