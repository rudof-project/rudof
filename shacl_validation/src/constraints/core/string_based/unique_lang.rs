use std::collections::HashSet;

use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::UniqueLang;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::lang::Lang;
use srdf::Literal;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for UniqueLang {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        if !self.unique_lang() {
            return Ok(Default::default());
        }

        let unique_lang = |targets: &FocusNodes<Q>| {
            let mut unique_langs = HashSet::new();
            let is_all_unique_langs = targets
                .iter()
                .flat_map(|term| {
                    term.clone()
                        .try_into()
                        .map_err(|_| ValidateError::ExpectedLiteral(term.to_string()))
                })
                .filter_map(|literal: Q::Literal| literal.lang().map(Lang::new_unchecked))
                .all(move |x| unique_langs.insert(x));
            Ok(!is_all_unique_langs)
        };

        validate_with(
            component,
            shape,
            value_nodes,
            FocusNodeIteration,
            unique_lang,
        )
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for UniqueLang {}
