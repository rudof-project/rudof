use std::fmt::Debug;
use std::ops::Not;

use shacl_ast::compiled::component::And;
use shacl_ast::compiled::component::CompiledComponent;
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

impl<Q: Query + Debug + 'static, E: Engine<Q>> Validator<Q, E> for And<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
        engine: E,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let and = |value_node: &Q::Term| {
            self.shapes()
                .iter()
                .all(|shape| {
                    let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
                    match shape.validate(store, &engine, Some(&focus_nodes)) {
                        Ok(results) => results.is_empty(),
                        Err(_) => false,
                    }
                })
                .not()
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, and)
    }
}

impl<S: Sparql + Query + Debug + 'static> SparqlValidator<S> for And<S> {}
