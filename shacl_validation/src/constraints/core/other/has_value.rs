use shacl_ast::value::Value;
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
use crate::value_nodes::ValueNodes;

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

impl<S: SRDFBasic, R: ValidatorRunner<S>> ConstraintComponent<S, R> for HasValue {
    fn evaluate(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        unimplemented!()
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for HasValue {
    fn evaluate_default(
        &self,
        validation_context: Arc<ValidationContext<S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for HasValue {
    fn evaluate_sparql(
        &self,
        validation_context: Arc<ValidationContext<S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
