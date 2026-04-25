use crate::error::ConstraintError;
use crate::ir::components::MinCount;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{Validator, validate_with};
use crate::validator::engine::Engine;
use crate::validator::iteration::FocusNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for MinCount {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        _: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        if self.min_count() == 0 {
            return Ok(Default::default());
        }

        validate_with(
            component,
            shape,
            value_nodes,
            FocusNodeIteration,
            |t| t.len() < self.min_count(),
            &format!("MinCount({}) not satisfied", self.min_count()),
            maybe_path,
        )
    }
}
