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
use crate::value_nodes::ValueNodes;

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

impl<S: SRDFBasic> ConstraintComponent<S> for LanguageIn {
    fn evaluate<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        let results = value_nodes.flat_map(move |(focus_node, value_node)| {
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
        });

        LazyValidationIterator::new(results)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for LanguageIn {
    fn evaluate_default<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for LanguageIn {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
