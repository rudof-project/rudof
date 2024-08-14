use std::collections::HashSet;

use crate::constraints::ConstraintResult;
use crate::shape::ValueNode;
use crate::validation_report::result::ValidationResult;

use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;

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

impl<S: SRDFBasic> ConstraintComponent<S> for UniqueLang {
    fn evaluate(
        &self,
        _: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        if !self.unique_lang {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();
        let mut langs = HashSet::new();

        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                if let Some(literal) = S::term_as_literal(value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if langs.contains(&lang) {
                            results.push(ValidationResult::new(
                                focus_node,
                                context,
                                Some(value_node),
                            ));
                        }
                        langs.insert(lang);
                    }
                }
            }
        }
        Ok(results)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for UniqueLang {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for UniqueLang {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
    ) -> ConstraintResult<S> {
        self.evaluate(executor, context, value_nodes)
    }
}
