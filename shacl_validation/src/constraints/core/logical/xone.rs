use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Xone;
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
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::shape::Validate;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<T: Triple> Validator<T> for Xone<S> {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        engine: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let xone = |value_node: &S::Term| {
            self.shapes()
                .iter()
                .filter(|shape| {
                    let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
                    match shape.validate(store, &engine, Some(&focus_nodes), subsetting) {
                        Ok(results) => results.is_empty(),
                        Err(_) => false,
                    }
                })
                .count()
                .ne(&1usize)
        };

        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            xone,
            subsetting,
        )
    }
}

impl<R: Rdf> NativeValidator<R> for Xone<R> {
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

impl<S: Sparql> SparqlValidator<S> for Xone<S> {
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
