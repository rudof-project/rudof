use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinInclusive;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::SHACLPath;
use srdf::Sparql;
use std::fmt::Debug;

impl<S: Query + Debug + 'static> NativeValidator<S> for MinInclusive<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_inclusive = |node: &S::Term| {
            let ord = store.compare(node, self.min_inclusive_value());
            println!(
                "Comparing {:?} with {:?}: {ord:?}",
                node,
                self.min_inclusive_value()
            );
            ord.map(|o| o.is_lt()).unwrap_or(true)
        };
        let message = format!("MinInclusive({}) not satisfied", self.min_inclusive_value());
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            min_inclusive,
            &message,
            maybe_path,
        )
    }
}

impl<S: Sparql + Debug + 'static> SparqlValidator<S> for MinInclusive<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_inclusive_value = self.min_inclusive_value().clone();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} <= {}) }} ",
                value_node, min_inclusive_value
            }
        };

        let message = format!("MinInclusive({}) not satisfied", self.min_inclusive_value());
        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query,
            &message,
            maybe_path,
        )
    }
}
