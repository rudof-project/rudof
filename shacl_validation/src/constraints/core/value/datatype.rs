use iri_s::IriS;
use prefixmap::IriRef;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::ValueNodes;

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

impl<S: SRDFBasic + 'static> ConstraintComponent<S> for Datatype<S> {
    fn evaluate(
        &self,
        _validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        let results = value_nodes
            .iter()
            .flat_map(move |(focus_node, value_node)| {
                if let Some(literal) = S::term_as_literal(value_node) {
                    if S::datatype(&literal) != self.datatype {
                        let result = ValidationResult::new(
                            focus_node,
                            &evaluation_context,
                            Some(value_node),
                        );
                        Some(result)
                    } else {
                        None
                    }
                } else {
                    let result =
                        ValidationResult::new(focus_node, &evaluation_context, Some(value_node));
                    Some(result)
                }
            })
            .collect::<Vec<_>>();

        Ok(LazyValidationIterator::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Datatype<S> {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Datatype<S> {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ConstraintError> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}
