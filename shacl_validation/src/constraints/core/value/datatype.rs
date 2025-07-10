use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Datatype;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Iri;
use srdf::Literal as _;
use srdf::Rdf;

use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<R: Rdf, E: Engine<R>> Validator<R, E> for Datatype<R> {
    fn validate(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        _store: &R,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let datatype = |value_node: &R::Term| {
            let literal: R::Literal = value_node
                .clone()
                .try_into()
                .map_err(|_| ValidateError::ExpectedLiteral(value_node.to_string()))?;
            Ok(literal.datatype() != self.datatype().as_str())
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, datatype)
    }
}
