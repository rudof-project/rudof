use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::MaxInclusive;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_sparql_ask;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<R: Rdf> NativeValidator<R> for MaxInclusive<R> {
    fn validate_native(
        &self,
        _component: &CompiledComponent<R>,
        _shape: &CompiledShape<R>,
        _store: &Store<R>,
        _value_nodes: &ValueNodes<R>,
        _subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        Err(ConstraintError::NotImplemented("MaxInclusive".to_string()))
    }
}

impl<R: Sparql> SparqlValidator<R> for MaxInclusive<R> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let max_inclusive_value = self.max_inclusive().clone();

        let query = |value_node: &R::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} >= {}) }} ",
                value_node, max_inclusive_value
            }
        };

        validate_sparql_ask(component, shape, store, value_nodes, query, subsetting)
    }
}
