use srdf::lang::Lang;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::ValueNodes;

/// The condition specified by sh:languageIn is that the allowed language tags
/// for each value node are limited by a given list of language tags.
///
/// https://www.w3.org/TR/shacl/#LanguageInConstraintComponent
pub(crate) struct LanguageIn {
    langs: Vec<Lang>,
}

impl LanguageIn {
    pub fn new(langs: Vec<Lang>) -> Self {
        LanguageIn { langs }
    }
}

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for LanguageIn {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        let results = value_nodes
            .iter()
            .flat_map(move |(focus_node, value_node)| {
                if let Some(literal) = S::term_as_literal(&value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if !self.langs.contains(&Lang::new(&lang)) {
                            let result = ValidationResult::new(
                                focus_node,
                                &evaluation_context,
                                Some(value_node),
                            );
                            Some(result)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    let result =
                        ValidationResult::new(focus_node, &evaluation_context, Some(value_node));
                    Some(result)
                }
            })
            .collect::<Vec<_>>();

        LazyValidationIterator::new(results.into_iter())
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for LanguageIn {
    fn evaluate_default<'a>(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for LanguageIn {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
