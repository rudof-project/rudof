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

/// sh:qualifiedValueShape specifies the condition that a specified number of
///  value nodes conforms to the given shape. Each sh:qualifiedValueShape can
///  have: one value for sh:qualifiedMinCount, one value for
///  sh:qualifiedMaxCount or, one value for each, at the same subject.
///
/// https://www.w3.org/TR/shacl/#QualifiedValueShapeConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct QualifiedValue {
    shape: RDFNode,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    qualified_value_shapes_disjoint: Option<bool>,
}

impl QualifiedValue {
    pub fn new(
        shape: RDFNode,
        qualified_min_count: Option<isize>,
        qualified_max_count: Option<isize>,
        qualified_value_shapes_disjoint: Option<bool>,
    ) -> Self {
        QualifiedValue {
            shape,
            qualified_min_count,
            qualified_max_count,
            qualified_value_shapes_disjoint,
        }
    }
}

impl<S: SRDFBasic, R: ValidatorRunner<S>> ConstraintComponent<S, R> for QualifiedValue {
    fn evaluate(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        unimplemented!()
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for QualifiedValue {
    fn evaluate_default(
        &self,
        validation_context: Arc<ValidationContext<S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for QualifiedValue {
    fn evaluate_sparql(
        &self,
        validation_context: Arc<ValidationContext<S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
