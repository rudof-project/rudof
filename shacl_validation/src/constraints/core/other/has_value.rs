use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::HasValue;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Rdf;
use srdf::Sparql;
use std::fmt::Debug;

impl<S: Rdf + Debug> Validator<S> for HasValue<S> {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let has_value =
            |targets: &FocusNodes<S>| !targets.iter().any(|value| value == self.value());
        validate_with(component, shape, value_nodes, FocusNodeIteration, has_value)
    }
}

impl<S: Query + Debug + 'static> NativeValidator<S> for HasValue<S> {
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

impl<S: Sparql + Debug + 'static> SparqlValidator<S> for HasValue<S> {
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
