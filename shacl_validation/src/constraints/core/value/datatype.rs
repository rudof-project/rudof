use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Datatype;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Iri;
use srdf::Literal as _;
use srdf::Query;
use srdf::Rdf;
use srdf::Sparql;
use std::fmt::Debug;

impl<S: Rdf + Debug> Validator<S> for Datatype<S> {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let datatype = |value_node: &S::Term| {
            let tmp: Result<S::Literal, _> = value_node.clone().try_into();
            if let Ok(literal) = tmp {
                return literal.datatype() != self.datatype().as_str();
            }
            true
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, datatype)
    }
}

impl<S: Query + Debug + 'static> NativeValidator<S> for Datatype<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape<S>>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            source_shape,
        )
    }
}

impl<S: Sparql + Debug + 'static> SparqlValidator<S> for Datatype<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape<S>>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            source_shape,
        )
    }
}
