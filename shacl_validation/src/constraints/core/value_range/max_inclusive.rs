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
use shacl_ir::compiled::component::MaxInclusive;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MaxInclusive {
    fn validate_native(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        _store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_inclusive = |node: &S::Term| match S::term_as_sliteral(node) {
            Ok(lit) => lit
                .partial_cmp(self.max_inclusive())
                .map(|o| o.is_gt())
                .unwrap_or(true),
            Err(_) => true,
        };
        let message = format!("MaxInclusive({}) not satisfied", self.max_inclusive());
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            max_inclusive,
            &message,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MaxInclusive {
    fn validate_sparql(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_inclusive_value = self.max_inclusive().clone();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} >= {}) }} ",
                value_node, max_inclusive_value
            }
        };

        let message = format!("MaxInclusive({}) not satisfied", self.max_inclusive());
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
