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
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// The property sh:uniqueLang can be set to true to specify that no pair of
///  value nodes may use the same language tag.
///
/// https://www.w3.org/TR/shacl/#UniqueLangConstraintComponent
pub(crate) struct UniqueLang {
    unique_lang: bool,
}

impl UniqueLang {
    pub fn new(unique_lang: bool) -> Self {
        UniqueLang { unique_lang }
    }
}

impl<S: SRDFBasic, R: ValidatorRunner<S>> ConstraintComponent<S, R> for UniqueLang {
    fn evaluate(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        if !self.unique_lang {
            return LazyValidationIterator::default();
        }

        let langs = Rc::new(RefCell::new(Vec::new()));

        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                let langs = Rc::clone(&langs);
                let mut langs = langs.borrow_mut();

                if let Some(literal) = S::term_as_literal(&value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if langs.contains(&lang) {
                            Some(ValidationResult::new(
                                &focus_node,
                                Arc::clone(&evaluation_context),
                                Some(&value_node),
                            ))
                        } else {
                            langs.push(lang);
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for UniqueLang {
    fn evaluate_default(
        &self,
        validation_context: Arc<ValidationContext<S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for UniqueLang {
    fn evaluate_sparql(
        &self,
        validation_context: Arc<ValidationContext<S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
