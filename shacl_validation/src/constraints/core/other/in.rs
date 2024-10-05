use shacl_ast::compiled::component::In;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::helpers::validate_with;
use crate::constraints::SparqlValidator;
use crate::constraints::{NativeValidator, Validator};
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: SRDFBasic> Validator<S> for In<S> {
    fn validate(
        &self,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let r#in = |value_node: &S::Term| !self.values().contains(value_node);
        validate_with(value_nodes, &ValueNodeIteration, r#in)
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for In<S> {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for In<S> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(store, SparqlEngine, value_nodes)
    }
}
