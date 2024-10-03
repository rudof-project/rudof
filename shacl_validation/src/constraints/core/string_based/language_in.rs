use shacl_ast::compiled::component::LanguageIn;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::value_nodes::ValueNodes;

impl<S: SRDFBasic> Validator<S> for LanguageIn<S> {
    fn validate(
        &self,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                if let Some(lang) = S::term_as_literal(value_node) {
                    if self.langs().contains(&lang) {
                        None
                    } else {
                        let result = ValidationResult::new(focus_node, Some(value_node));
                        Some(result)
                    }
                } else {
                    let result = ValidationResult::new(focus_node, Some(value_node));
                    Some(result)
                }
            });

        Ok(ValidationResults::new(results))
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for LanguageIn<S> {
    fn validate_native<'a>(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for LanguageIn<S> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, SparqlEngine, value_nodes)
    }
}
