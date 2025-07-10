use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::LanguageIn;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::lang::Lang;
use srdf::Literal;
use srdf::Rdf;

use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<R: Rdf, E: Engine<R>> Validator<R, E> for LanguageIn {
    fn validate(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        _store: &R,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let language_in = |value_node: &R::Term| {
            let literal: R::Literal = value_node
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
