use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Node;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Rdf;

use crate::constraints::Validator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::shape::Validate;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<R: Rdf, E: Engine<R>> Validator<R, E> for Node<R> {
    fn validate(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &R,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let node = |value_node: &R::Term| {
            let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
            match Validate::<R>::validate::<E>(self.shape(), store, Some(&focus_nodes)) {
                Ok(results) => Ok(!results.is_empty()),
                Err(error) => Err(error),
            }
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, node)
    }
}
