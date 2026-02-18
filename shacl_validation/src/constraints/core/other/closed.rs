use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF};
use shacl_ir::compiled::component_ir::Closed;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Closed {
    fn validate(
        &self,
        _component: &ComponentIR,
        _shape: &ShapeIR,
        _store: &S,
        _engine: &mut dyn Engine<S>,
        _value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        _maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Closed".to_string()))
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Closed {
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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Closed {
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
