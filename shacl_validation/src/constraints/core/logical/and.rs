use shacl_ast::compiled::component::And;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::Validator;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::context::Context;
use crate::shape::ShapeValidation;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::Targets;
use crate::ValueNodes;

impl<S: SRDFBasic + 'static> Validator<S> for And<S> {
    fn validate(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                let all_valid = self.shapes().iter().all(|shape| {
                    let focus_nodes = Targets::new(std::iter::once(value_node.clone()));
                    let shape_validator =
                        ShapeValidation::new(shape, validation_context, Some(&focus_nodes));

                    match shape_validator.validate() {
                        Ok(results) => results.is_empty(),
                        Err(_) => false,
                    }
                });

                if !all_valid {
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

impl<S: SRDF + 'static> NativeValidator<S> for And<S> {
    fn validate_native(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for And<S> {
    fn validate_sparql(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(evaluation_context, value_nodes)
    }
}
