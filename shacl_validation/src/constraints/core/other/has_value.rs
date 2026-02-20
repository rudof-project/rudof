use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::FocusNodeIteration;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF};
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::HasValue;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for HasValue {
    fn validate(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        _: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let has_value = |targets: &FocusNodes<S>| {
            let value_term = &S::object_as_term(self.value());
            !targets.iter().any(|value| value == value_term)
        };
        let message = format!("HasValue({}) not satisfied", self.value());
        validate_with(
            component,
            shape,
            value_nodes,
            FocusNodeIteration,
            has_value,
            &message,
            maybe_path,
        )
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for HasValue {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            engine,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for HasValue {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            &mut SparqlEngine::new(),
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}
