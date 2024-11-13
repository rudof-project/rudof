use std::fmt::Debug;

use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Pattern;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::helpers::constraint::validate_sparql_ask;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<S: SRDF + Debug + 'static> NativeValidator<S> for Pattern {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let pattern = |value_node: &S::Term| {
            if S::term_is_bnode(value_node) {
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

impl<S: Sparql> SparqlValidator<S> for Pattern {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let flags = self.flags().clone();
        let pattern = self.pattern().clone();

        let query = |value_node: &S::Term| match &flags {
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
