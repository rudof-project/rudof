use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::LanguageIn;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::lang::Lang;
use srdf::Literal;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for LanguageIn {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let language_in = |value_node: &Q::Term| {
            if let Ok(literal) = value_node.clone().try_into() {
                let literal: Q::Literal = literal;
                return match literal.lang() {
                    Some(lang) => !self.langs().contains(&Lang::new_unchecked(lang)),
                    None => true,
                };
            }
            true
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
