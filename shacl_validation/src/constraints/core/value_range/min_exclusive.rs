use std::collections::HashSet;

use indoc::formatdoc;
use shacl_ast::Schema;
use srdf::literal::Literal;
use srdf::{QuerySRDF, RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::validation_report::report::ValidationReport;

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
pub(crate) struct MinExclusive<S: SRDFBasic> {
    min_inclusive: S::Term,
}

impl<S: SRDFBasic> MinExclusive<S> {
    pub fn new(literal: Literal) -> Self {
        MinExclusive {
            min_inclusive: S::object_as_term(&RDFNode::literal(literal)),
        }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for MinExclusive<S> {
    fn evaluate_default(
        &self,
        _store: &S,
        _: &Schema,
        _: &DefaultValidatorRunner,
        _value_nodes: &HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for MinExclusive<S> {
    fn evaluate_sparql(
        &self,
        store: &S,
        _: &Schema,
        _: &SparqlValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for node in value_nodes {
            let query = formatdoc! {
                " ASK {{ FILTER ({} < {}) }} ",
                node, self.min_inclusive
            };
            let ask = match store.query_ask(&query) {
                Ok(ask) => ask,
                Err(_) => return Err(ConstraintError::Query),
            };
            if !ask {
                ans = false;
                report.make_validation_result(Some(node));
            }
        }
        Ok(ans)
    }
}
