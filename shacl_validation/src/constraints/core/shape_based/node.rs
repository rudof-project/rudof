use shacl_ast::compiled::component::Node;
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
use crate::focus_nodes::FocusNodes;
use crate::shape::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<S: SRDFBasic> Validator<S> for Node<S> {
    fn validate(
        &self,
        store: &S,
        engine: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
                let inner_results = self.shape().validate(store, &engine, Some(&focus_nodes));
                if inner_results.is_err() || !inner_results.unwrap().is_empty() {
                    // in case of error or that there exists any result...
                    Some(ValidationResult::new(focus_node, Some(value_node)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for Node<S> {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Node<S> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(store, SparqlEngine, value_nodes)
    }
}
