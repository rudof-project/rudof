use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::HasValue;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Rdf;

use crate::constraints::Validator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;

impl<R: Rdf, E: Engine<R>> Validator<R, E> for HasValue<R> {
    fn validate(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        _store: &R,
        value_nodes: &ValueNodes<R>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        // TODO: can this be changed to ValueNodeIteration?
        let has_value =
            |targets: &FocusNodes<R>| Ok(!targets.iter().any(|value| value == self.value()));
        validate_with(component, shape, value_nodes, FocusNodeIteration, has_value)
    }
}
