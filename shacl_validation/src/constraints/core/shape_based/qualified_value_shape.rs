use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::QualifiedValueShape;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<T: Triple> Validator<T> for QualifiedValueShape<S> {
    fn validate(
        &self,
        _component: &CompiledComponent<S>,
        _shape: &CompiledShape<S>,
        _store: &Store<S>,
        _engine: impl Engine<S>,
        _value_nodes: &ValueNodes<S>,
        _subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        Err(ConstraintError::NotImplemented(
            "QualifiedValueShape".to_string(),
        ))
    }
}

impl<R: Rdf> NativeValidator<R> for QualifiedValueShape<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            subsetting,
        )
    }
}

impl<S: Sparql> SparqlValidator<S> for QualifiedValueShape<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            subsetting,
        )
    }
}
