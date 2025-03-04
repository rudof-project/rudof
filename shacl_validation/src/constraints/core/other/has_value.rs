use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::HasValue;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for HasValue<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let has_value =
            |targets: &FocusNodes<Q>| Ok(!targets.iter().any(|value| value == self.value()));
        validate_with(component, shape, value_nodes, FocusNodeIteration, has_value)
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for HasValue<S> {}
