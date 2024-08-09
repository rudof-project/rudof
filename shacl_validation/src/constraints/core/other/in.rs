use std::collections::HashSet;

use shacl_ast::value::Value;
use shacl_ast::Schema;
use srdf::{QuerySRDF, SRDF};
use srdf::{RDFNode, SRDFBasic};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlConstraintComponent;
use crate::constraints::{ConstraintComponent, DefaultConstraintComponent};
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::validation_report::report::ValidationReport;

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
pub(crate) struct In<S: SRDFBasic> {
    values: Vec<S::Term>,
}

impl<S: SRDFBasic> In<S> {
    pub fn new(values: Vec<Value>) -> Self {
        In {
            values: values
                .iter()
                .map(|value| match value {
                    Value::Iri(iri_ref) => S::iri_s2term(&iri_ref.get_iri().unwrap()),
                    Value::Literal(lit) => S::object_as_term(&RDFNode::literal(lit.to_owned())),
                })
                .collect(),
        }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for In<S> {
    fn evaluate(
        &self,
        _: &S,
        _: &Schema,
        _: &dyn ValidatorRunner<S>,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let ans = value_nodes.iter().all(|node| {
            if !self.values.contains(node) {
                report.make_validation_result(Some(node));
                false
            } else {
                true
            }
        });

        Ok(ans)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for In<S> {
    fn evaluate_default(
        &self,
        store: &S,
        schema: &Schema,
        runner: &DefaultValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(store, schema, runner, value_nodes, report)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for In<S> {
    fn evaluate_sparql(
        &self,
        store: &S,
        schema: &Schema,
        runner: &SparqlValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        self.evaluate(store, schema, runner, value_nodes, report)
    }
}
