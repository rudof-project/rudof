use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MinLength;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::TObject;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;
use srdf::model::Literal;
use srdf::model::Term;

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

impl<R: Rdf + Clone + 'static, E: Engine<R>> NativeValidator<R, E> for MinLength {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let min_length = |value_node: &TObject<R>| {
            if value_node.is_blank_node() {
                true
            } else {
                let string_representation = match value_node.as_literal() {
                    Some(string_representation) => string_representation.as_string().unwrap(),
                    None => value_node.as_iri().unwrap().to_string(),
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
            subsetting,
        )
    }
}

impl<S: Rdf + Sparql + Clone + 'static> SparqlValidator<S> for MinLength {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let min_length_value = self.min_length();

        let query = |value_node: &TObject<S>| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                value_node, min_length_value
            }
        };

        validate_sparql_ask(component, shape, store, value_nodes, query, subsetting)
    }
}
