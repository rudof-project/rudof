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
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Not;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::Rdf;
use srdf::SRDF;
use std::fmt::Debug;

impl<S: Rdf + Debug> Validator<S> for Not<S> {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        engine: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let not = |value_node: &S::Term| {
            let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
            let inner_results = self.shape().validate(store, &engine, Some(&focus_nodes));
            inner_results.is_err() || inner_results.unwrap().is_empty()
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, not)
    }
}

impl<S: SRDF + Debug + 'static> NativeValidator<S> for Not<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + Debug + 'static> SparqlValidator<S> for Not<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(component, shape, store, SparqlEngine, value_nodes)
    }
}
