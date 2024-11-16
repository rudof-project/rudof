use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::sparql::SparqlEngine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

use crate::engine::Engine;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinCount;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;

impl<R: Rdf + Clone + 'static, E: Engine<R>> NativeValidator<R, E> for MinCount {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        // If min_count is 0, then it always passes
        if self.min_count() == 0 {
            return Ok(Default::default());
        }

        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            FocusNodeIteration,
            |targets: &FocusNodes<R>| targets.len() < self.min_count() as usize,
            subsetting,
        )
    }
}

impl<S: Rdf + Sparql + Clone + 'static> SparqlValidator<S> for MinCount {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate_native(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            subsetting,
        )
    }
}
