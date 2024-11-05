use std::fmt::Debug;
use std::ops::Not;

use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Or;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::shape::Validate;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: SRDFBasic + Debug> Validator<S> for Or<S> {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        engine: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let or = |value_node: &S::Term| {
            self.shapes()
                .iter()
                .any(|shape| {
                    match shape.validate(
                        store,
                        &engine,
                        Some(&FocusNodes::new(std::iter::once(value_node.clone()))),
                    ) {
                        Ok(validation_results) => validation_results.is_empty(),
                        Err(_) => false,
                    }
                })
                .not()
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, or)
    }
}

impl<S: SRDF + Debug + 'static> NativeValidator<S> for Or<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + Debug + 'static> SparqlValidator<S> for Or<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, SparqlEngine, value_nodes)
    }
}
