use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::value_nodes::ValueNodes;

use shacl_ast::compiled::component::MinCount;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

impl<S: SRDFBasic> Validator<S> for MinCount {
    fn validate(
        &self,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        if self.min_count() == 0 {
            // If min_count is 0, then it always passes
            return Ok(ValidationResults::default());
        }

        let results = value_nodes
            .iter_focus_nodes()
            .filter_map(|(focus_node, targets)| {
                if targets.len() < self.min_count() {
                    Some(ValidationResult::new(focus_node, None))
                } else {
                    None
                }
            });

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for MinCount {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MinCount {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, SparqlEngine, value_nodes)
    }
}
