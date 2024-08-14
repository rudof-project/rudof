use srdf::lang::Lang;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::constraints::ConstraintResult;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::shape::ValueNode;
use crate::validation_report::result::ValidationResult;

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
    fn evaluate(
        &self,
        _executor: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        let mut results = Vec::new();

        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                if let Some(literal) = S::term_as_literal(value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if !self.langs.contains(&Lang::new(&lang)) {
                            results.push(ValidationResult::new(
                                focus_node,
                                context,
                                Some(value_node),
                            ));
                        }
                    }
                } else {
                    results.push(ValidationResult::new(focus_node, context, Some(value_node)));
                }
            }
        }

        Ok(results)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for LanguageIn {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for LanguageIn {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}
