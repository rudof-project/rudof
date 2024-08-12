use iri_s::IriS;
use prefixmap::IriRef;
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
use crate::shape::ValueNode;
use crate::validation_report::report::ValidationReport;

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Datatype<S: SRDFBasic> {
    datatype: S::IRI,
}

impl<S: SRDFBasic> Datatype<S> {
    pub fn new(iri_ref: IriRef) -> Self {
        Datatype {
            datatype: S::iri_s2iri(&IriS::new_unchecked(&iri_ref.to_string())),
        }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for Datatype<S> {
    fn evaluate(
        &self,
        _executor: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                if let Some(literal) = S::term_as_literal(value_node) {
                    if S::datatype(&literal) != self.datatype {
                        ans = false;
                        report.make_validation_result(focus_node, context, Some(value_node));
                    }
                } else {
                    ans = false;
                    report.make_validation_result(focus_node, context, Some(value_node));
                }
            }
        }
        Ok(ans)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Datatype<S> {
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

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Datatype<S> {
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
