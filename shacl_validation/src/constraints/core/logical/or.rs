use std::fmt::Debug;
use std::ops::Not;

use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Or;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::shape::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for Or<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
        engine: E,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let or = |value_node: &Q::Term| {
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

impl<S: Sparql + Query> SparqlValidator<S> for Or<S> {}
