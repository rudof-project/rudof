use shacl_ast::compiled::component::MaxCount;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

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

impl<S: SRDFBasic> Validator<S> for MaxCount {
    fn validate(
        &self,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_focus_nodes()
            .filter_map(|(focus_node, targets)| {
                if targets.len() > self.max_count() {
                    Some(ValidationResult::new(focus_node, None))
                } else {
                    None
                }
            });

        Ok(ValidationResults::new(results))
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for MaxCount {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for MaxCount {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, SparqlEngine, value_nodes)
    }
}
