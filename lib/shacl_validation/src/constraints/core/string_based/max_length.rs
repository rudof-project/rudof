use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MaxLength;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;
use srdf::model::Literal as _;
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

impl<R: Rdf, E: Engine<R>> NativeValidator<R, E> for MaxLength {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let max_length = |value_node: &Object<R>| {
            if value_node.is_blank_node() {
                true
            } else {
                let string_representation = match value_node.literal() {
                    Some(string_representation) => string_representation.as_string().unwrap(),
                    None => value_node.iri().unwrap().to_string(),
                };
                string_representation.len() > self.max_length() as usize
            }
        };

        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            max_length,
            subsetting,
        )
    }
}

impl<S: Rdf + Sparql> SparqlValidator<S> for MaxLength {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let max_length_value = self.max_length();

        let query = |value_node: &Object<S>| {
            formatdoc! {
                " ASK {{ FILTER (STRLEN(str({})) <= {}) }} ",
                value_node, max_length_value
            }
        };

        validate_sparql_ask(component, shape, store, value_nodes, query, subsetting)
    }
}
