use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Xone;
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

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for Xone<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let xone = |value_node: &Q::Term| {
            let valid_count = self
                .shapes()
                .iter()
                .filter(|&xone_shape| {
                    let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
                    match Validate::<Q>::validate::<E>(xone_shape, store, Some(&focus_nodes)) {
                        Ok(results) => results.is_empty(),
                        Err(_) => false,
                    }
                })
                .count();

            println!("valid_count: {valid_count}");

            valid_count != 1usize
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, xone)
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for Xone<S> {}
