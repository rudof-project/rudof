use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Pattern;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;
use srdf::model::Term as _;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::helpers::constraint::validate_sparql_ask;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<R: Rdf, E: Engine<R>> NativeValidator<R, E> for Pattern {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let pattern = |value_node: &Object<R>| {
            if value_node.is_blank_node() {
                true
            } else {
                todo!()
            }
        };
        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            pattern,
            subsetting,
        )
    }
}

impl<S: Rdf + Sparql> SparqlValidator<S> for Pattern {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let flags = self.flags().clone();
        let pattern = self.pattern().clone();

        let query = |value_node: &Object<S>| match &flags {
            Some(flags) => formatdoc! {
                "ASK {{ FILTER (regex(str({}), {}, {})) }}",
                value_node, pattern, flags
            },
            None => formatdoc! {
                "ASK {{ FILTER (regex(str({}), {})) }}",
                value_node, pattern
            },
        };

        validate_sparql_ask(component, shape, store, value_nodes, query, subsetting)
    }
}
