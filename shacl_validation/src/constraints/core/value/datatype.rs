use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Datatype;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Iri;
use srdf::Literal as _;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for Datatype<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let datatype = |value_node: &Q::Term| {
            let literal: Q::Literal = value_node
                .clone()
                .try_into()
                .map_err(|_| ValidateError::ExpectedLiteral(value_node.to_string()))?;
            Ok(literal.datatype() != self.datatype().as_str())
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, datatype)
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for Datatype<S> {}
