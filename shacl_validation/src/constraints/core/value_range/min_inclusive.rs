use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::MinInclusive;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MinInclusive<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        _store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_inclusive = |node: &S::Term| match S::term_as_sliteral(node) {
            Ok(lit) => lit
                .partial_cmp(self.min_inclusive_value())
                .map(|o| o.is_lt())
                .unwrap_or(true),
            Err(_) => true,
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

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MinInclusive<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
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
