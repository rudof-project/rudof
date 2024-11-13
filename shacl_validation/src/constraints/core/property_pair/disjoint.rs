use std::fmt::Debug;

use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Disjoint;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<R: Rdf> NativeValidator<R> for Disjoint<S> {
    fn validate_native(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &Store<S>,
        _value_nodes: &ValueNodes<S>,
        _subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Disjoint".to_string()))
    }
}

impl<S: Sparql> SparqlValidator<S> for Disjoint<S> {
    fn validate_sparql(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &Store<S>,
        _value_nodes: &ValueNodes<S>,
        _subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Disjoint".to_string()))
    }
}
