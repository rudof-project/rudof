use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Disjoint;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::Engine;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<R: Rdf, E: Engine<R>> NativeValidator<R, E> for Disjoint<R> {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Disjoint".to_string()))
    }
}

impl<S: Rdf + Sparql> SparqlValidator<S> for Disjoint<S> {
    fn validate_sparql(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &Store<S>,
        _value_nodes: &ValueNodes<S>,
        _subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Disjoint".to_string()))
    }
}
