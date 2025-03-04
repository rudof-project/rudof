use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::LanguageIn;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::lang::Lang;
use srdf::Literal;
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

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for LanguageIn {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let language_in = |value_node: &Q::Term| {
            let literal: Q::Literal = value_node
                .clone()
                .try_into()
                .map_err(|_| ValidateError::ExpectedLiteral(value_node.to_string()))?;
            match literal.lang() {
                Some(lang) => Ok(!self.langs().contains(&Lang::new_unchecked(lang))),
                None => Ok(true),
            }
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            language_in,
        )
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for LanguageIn {}
