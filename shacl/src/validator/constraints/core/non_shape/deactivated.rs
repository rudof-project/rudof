use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use crate::ir::components::Deactivated;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validator::engine::{Engine, SparqlEngine};
use crate::validator::report::ValidationResult;
use crate::validator::nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Deactivated {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        // If is deactivated this shouldn't be reached
        // If is activated, no error should be raised
        Ok(Vec::new())
    }
}
