use shacl_ast::value::Value;
use srdf::QuerySRDF;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlConstraintComponent;
use crate::constraints::{ConstraintComponent, DefaultConstraintComponent};
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::shape::ValueNode;
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
        _: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let ans = value_nodes.iter().all(|(focus_node, value_nodes)| {
            value_nodes.iter().all(|value_node| {
                if !self.values.contains(value_node) {
                    report.make_validation_result(focus_node, context, Some(value_node));
                    false
                } else {
                    true
                }
            })
        });

        Ok(ans)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for In<S> {
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

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for In<S> {
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
