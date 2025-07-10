use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::In;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Rdf;

use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<R: Rdf, E: Engine<R>> Validator<R, E> for In<R> {
    fn validate(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        _store: &R,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let r#in = |value_node: &R::Term| Ok(!self.values().contains(value_node));
        validate_with(component, shape, value_nodes, ValueNodeIteration, r#in)
    }
}
