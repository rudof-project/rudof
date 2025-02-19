use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Or;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::shape::Validate;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<R: Rdf, E: Engine<R>> NativeValidator<R, E> for Or<R> {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let or = |value_node: &Object<R>| {
            let any = self.shapes().iter().any(|shape| {
                let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
                match shape.validate(store, &engine, Some(&focus_nodes), subsetting) {
                    Ok(validation_results) => validation_results.is_empty(),
                    Err(_) => false,
                }
            });
            !any
        };

        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            or,
            subsetting,
        )
    }
}

impl<S: Rdf + Sparql> SparqlValidator<S> for Or<S> {
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
