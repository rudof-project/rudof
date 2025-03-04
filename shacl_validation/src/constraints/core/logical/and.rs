use shacl_ast::compiled::component::And;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::shape::Validate;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for And<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let and = |value_node: &Q::Term| {
            let is_all_valid = self.shapes().iter().all(|and_shape| {
                let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
                match Validate::<Q>::validate::<E>(and_shape, store, Some(&focus_nodes)) {
                    Ok(results) => results.is_empty(),
                    Err(_) => false, // TODO: return an error here
                }
            });
            Ok(!is_all_valid)
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, and)
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for And<S> {}
