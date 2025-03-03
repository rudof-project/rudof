use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Node;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::shape::Validate;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for Node<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let node = |value_node: &Q::Term| {
            let focus_nodes = FocusNodes::new(std::iter::once(value_node.clone()));
            let results = Validate::<Q>::validate::<E>(self.shape(), store, Some(&focus_nodes));
            results.is_err() || !results.unwrap().is_empty()
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, node)
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for Node<S> {}
