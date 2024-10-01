use shacl_ast::compiled::component::Node;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::context::Context;
use crate::shape::ShapeValidation;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::Targets;
use crate::ValueNodes;

impl<S: SRDFBasic + 'static> Validator<S> for Node<S> {
    fn validate(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                let focus_nodes = Targets::new(std::iter::once(value_node.clone()));
                let inner_results =
                    ShapeValidation::new(self.shape(), Some(&focus_nodes)).validate();
                if inner_results.is_err() || inner_results.unwrap().is_empty() {
                    Some(ValidationResult::new(
                        focus_node,
                        &evaluation_context,
                        Some(value_node),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for Node<S> {
    fn validate_native(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Node<S> {
    fn validate_sparql(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(evaluation_context, value_nodes)
    }
}
