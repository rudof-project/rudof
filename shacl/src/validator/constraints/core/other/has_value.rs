use crate::ir::components::HasValue;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{ConstraintError, Validator, validate_with};
use crate::validator::engine::Engine;
use crate::validator::iteration::FocusNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for HasValue {
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
        validate_with(
            component,
            shape,
            value_nodes,
            FocusNodeIteration,
            |t| {
                let value_term = &S::object_as_term(self.value());
                !t.iter().any(|v| v == value_term)
            },
            &format!("HasValue({}) not satisfied", self.value()),
            maybe_path,
        )
    }
}
