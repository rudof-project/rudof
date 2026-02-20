use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shacl_engine::engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF};
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::MaxExclusive;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MaxExclusive {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        _store: &S,
        _engine: &mut dyn engine::Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_exclusive = |node: &S::Term| match S::term_as_sliteral(node) {
            Ok(lit) => lit.partial_cmp(self.max_exclusive()).map(|o| o.is_ge()).unwrap_or(true),
            Err(_) => true,
        };
        let message = format!("MaxExclusive({}) not satisfied", self.max_exclusive());
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            max_exclusive,
            &message,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MaxExclusive {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let max_exclusive_value = self.max_exclusive().clone();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} > {}) }} ",
                value_node, max_exclusive_value
            }
        };

        let message = format!("MaxExclusive({}) not satisfied", self.max_exclusive());
        validate_ask_with(component, shape, store, value_nodes, query, &message, maybe_path)
    }
}
