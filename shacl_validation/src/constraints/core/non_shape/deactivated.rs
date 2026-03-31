use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use shacl::ir::components::Deactivated;
use shacl::ir::{IRComponent, IRSchema, IRShape};
use crate::constraints::{NativeValidator, SparqlValidator, Validator};
use crate::constraints::constraint_error::ConstraintError;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Deactivated {
    fn validate(
        &self,
        _component: &IRComponent,
        _shape: &IRShape,
        _store: &S,
        _engine: &mut dyn Engine<S>,
        _value_nodes: &ValueNodes<S>,
        _source_shape: Option<&IRShape>,
        _maybe_path: Option<&SHACLPath>,
        _shapes_graph: &IRSchema
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        // If is deactivated this shouldn't be reached
        // If is activated, no error should be raised
        Ok(Vec::new())
    }
}

impl<S: NeighsRDF + Debug> NativeValidator<S> for Deactivated {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            engine,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph
        )
    }
}

impl<S: QueryRDF + NeighsRDF + Debug +'static> SparqlValidator<S> for Deactivated {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            &mut SparqlEngine::new(),
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph
        )
    }
}