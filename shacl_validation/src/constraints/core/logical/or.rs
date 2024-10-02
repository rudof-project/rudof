use shacl_ast::compiled::component::Or;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::runner::native::NativeValidatorRunner;
use crate::runner::sparql::SparqlValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::shape::ShapeValidation;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::Targets;
use crate::ValueNodes;

impl<S: SRDFBasic + 'static> Validator<S> for Or<S> {
    fn validate(
        &self,
        store: &S,
        runner: impl ValidatorRunner<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                let any_valid = self.shapes().iter().any(|shape| {
                    let focus_nodes = Targets::new(std::iter::once(value_node.clone()));
                    let validation =
                        ShapeValidation::new(store, &runner, shape, Some(&focus_nodes));

                    match validation.validate() {
                        Ok(results) => results.is_empty(),
                        Err(_) => false,
                    }
                });

                if !any_valid {
                    Some(ValidationResult::new(focus_node, Some(value_node)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for Or<S> {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, NativeValidatorRunner, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Or<S> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, SparqlValidatorRunner, value_nodes)
    }
}
