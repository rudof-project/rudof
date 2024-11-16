use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Datatype;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Object;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;
use srdf::model::Iri;
use srdf::model::Literal;
use srdf::model::Term;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<R: Rdf + Clone + 'static, E: Engine<R>> NativeValidator<R, E> for Datatype<R> {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let datatype = |value_node: &Object<R>| {
            if let Some(literal) = value_node.as_literal() {
                return Predicate::<R>::new(literal.datatype()) != *self.datatype();
            }
            true
        };

        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            datatype,
            subsetting,
        )
    }
}

impl<S: Rdf + Sparql + Clone + 'static> SparqlValidator<S> for Datatype<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate_native(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            subsetting,
        )
    }
}
