use prefixmap::IriRef;
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

/// The RDF data model offers a huge amount of flexibility. Any node can in
/// principle have values for any property. However, in some cases it makes
/// sense to specify conditions on which properties can be applied to nodes.
/// The SHACL Core language includes a property called sh:closed that can be
/// used to specify the condition that each value node has values only for
/// those properties that have been explicitly enumerated via the property
/// shapes specified for the shape via sh:property.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct Closed {
    is_closed: bool,
    ignored_properties: Vec<IriRef>,
}

impl Closed {
    pub fn new(is_closed: bool, ignored_properties: Vec<IriRef>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }
}

impl<S: SRDFBasic, R: ValidatorRunner<S>> ConstraintComponent<S, R> for Closed {
    fn evaluate(
        &self,
        validation_context: Arc<ValidationContext<S, R>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        unimplemented!()
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for Closed {
    fn evaluate_default(
        &self,
        validation_context: Arc<ValidationContext<S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Closed {
    fn evaluate_sparql(
        &self,
        validation_context: Arc<ValidationContext<S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext>,
        value_nodes: Arc<ValueNodes<S>>,
    ) -> LazyValidationIterator<S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
