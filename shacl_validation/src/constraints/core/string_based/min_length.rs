use std::fmt::Debug;

use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinLength;
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

impl<S: SRDF + Debug + 'static> NativeValidator<S> for MinLength {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &Store<S>,
        value_nodes: &ValueNodes<S>,
        _subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_length = |value_node: &S::Term| {
            if S::term_is_bnode(value_node) {
                true
            } else {
                let string_representation = match S::term_as_string(value_node) {
                    Some(string_representation) => string_representation,
                    None => S::iri2iri_s(S::term_as_iri(value_node).unwrap()).to_string(),
                };
                string_representation.len() < self.min_length() as usize
            }
        };

        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            min_length,
        )
    }
}

impl<S: QuerySRDF + Debug + 'static> SparqlValidator<S> for MinLength {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        _subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let min_length_value = self.min_length();

        let query = |value_node: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                value_node, min_length_value
            }
        };

        validate_sparql_ask(component, shape, store, value_nodes, query)
    }
}
