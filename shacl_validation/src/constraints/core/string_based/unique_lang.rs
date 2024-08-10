use std::collections::HashSet;

use crate::shape::ValueNode;

use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::validation_report::report::ValidationReport;

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
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        if !self.unique_lang {
            return Ok(true);
        }
        let mut ans = true;
        let mut langs = HashSet::new();
        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                if let Some(literal) = S::term_as_literal(value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if langs.contains(&lang) {
                            ans = false;
                            report.make_validation_result(focus_node, context, Some(value_node));
                        }
                        langs.insert(lang);
                    }
                }
            }
        }
        Ok(ans)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for UniqueLang {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(executor, context, value_nodes, report)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for UniqueLang {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(executor, context, value_nodes, report)
    }
}
