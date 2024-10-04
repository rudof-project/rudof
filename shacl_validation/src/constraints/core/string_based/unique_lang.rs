use shacl_ast::compiled::component::UniqueLang;
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
use crate::value_nodes::ValueNodes;

impl<S: SRDFBasic> Validator<S> for UniqueLang {
    fn validate(
        &self,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        if !self.unique_lang() {
            return Ok(Vec::default());
        }

        let mut langs = Vec::new();

        let results = value_nodes
            .iter_value_nodes()
            .filter_map(|(focus_node, value_node)| {
                if let Some(literal) = S::term_as_literal(value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if langs.contains(&lang) {
                            return Some(ValidationResult::new(focus_node, Some(value_node)));
                        } else {
                            langs.push(lang);
                        }
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for UniqueLang {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for UniqueLang {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(store, SparqlEngine, value_nodes)
    }
}
