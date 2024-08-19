use srdf::lang::Lang;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;
use std::sync::Arc;

use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
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

impl< S: SRDFBasic, R: ValidatorRunner< S>> ConstraintComponent< S, R> for LanguageIn {
    fn evaluate(
        & self,
        validation_context: Arc<ValidationContext< S, R>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                if let Some(literal) = S::term_as_literal(&value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if !self.langs.contains(&Lang::new(&lang)) {
                            let result = ValidationResult::new(
                                &focus_node,
                                Arc::clone(&evaluation_context),
                                Some(&value_node),
                            );
                            Some(result)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    let result = ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        Some(&value_node),
                    );
                    Some(result)
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl< S: SRDF> DefaultConstraintComponent< S> for LanguageIn {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for LanguageIn {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
