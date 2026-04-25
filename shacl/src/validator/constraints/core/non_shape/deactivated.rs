use crate::ir::components::Deactivated;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{ConstraintError, Validator};
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Deactivated {
    fn validate(
        &self,
        _: &IRComponent,
        _: &IRShape,
        _: &S,
        _: &mut dyn Engine<S>,
        _: &ValueNodes<S>,
        _: Option<&IRShape>,
        _: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        // If is deactivated this shouldn't be reached
        // If is activated, no error should be raised
        Ok(Vec::new())
    }
}
