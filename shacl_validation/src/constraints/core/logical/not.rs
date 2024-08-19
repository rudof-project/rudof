use srdf::QuerySRDF;
use srdf::RDFNode;
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

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Not {
    shape: RDFNode,
}

impl Not {
    pub fn new(shape: RDFNode) -> Self {
        Not { shape }
    }
}

impl< S: SRDFBasic, R: ValidatorRunner< S>> ConstraintComponent< S, R> for Not {
    fn evaluate(
        & self,
        validation_context: Arc<ValidationContext< S, R>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        unimplemented!()
    }
}

impl< S: SRDF> DefaultConstraintComponent< S> for Not {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for Not {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
