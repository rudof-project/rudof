use shacl_ast::value::Value;
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

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct HasValue {
    value: Value,
}

impl HasValue {
    pub fn new(value: Value) -> Self {
        HasValue { value }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for HasValue {
    fn evaluate(
        &self,
        _executor: &dyn SHACLExecutor<S>,
        _context: &Context,
        _value_nodes: &ValueNode<S>,
        _report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for HasValue {
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

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for HasValue {
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
