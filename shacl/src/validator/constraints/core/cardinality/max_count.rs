use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use crate::error::ConstraintError;
use crate::ir::components::MaxCount;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{validate_with, Validator};
use crate::validator::engine::Engine;
use crate::validator::iteration::FocusNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;

impl<S: NeighsRDF + Debug> Validator<S> for MaxCount {
    fn validate(&self, component: &IRComponent, shape: &IRShape, _: &S, _: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, _: Option<&IRShape>, maybe_path: Option<&SHACLPath>, _: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        validate_with(
            component,
            shape,
            value_nodes,
            FocusNodeIteration,
            |t| t.len() > self.max_count(),
            &format!("MaxCount({}) not satisfied", self.max_count()),
            maybe_path
        )
    }
}
