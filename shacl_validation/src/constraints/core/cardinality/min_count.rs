use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinCount;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for MinCount {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        // If min_count is 0, then it always passes
        if self.min_count() == 0 {
            return Ok(Default::default());
        }
        let min_count = |targets: &FocusNodes<Q>| targets.len() < self.min_count();
        validate_with(component, shape, value_nodes, FocusNodeIteration, min_count)
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for MinCount {}
