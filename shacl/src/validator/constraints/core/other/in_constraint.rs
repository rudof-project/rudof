use crate::error::ValidationError;
use crate::ir::components::In;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{Validator, validate_with};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationOutcome;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for In {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        _: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<ValidationOutcome, ValidationError> {
        let prefixmap = shapes_graph.prefix_map_for_messages();
        let expected = self
            .values()
            .iter()
            .map(|value| value.show_qualified(&prefixmap))
            .collect::<Vec<_>>()
            .join(", ");
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |vn| {
                let values = self.values().iter().map(S::object_as_term).collect::<Vec<_>>();
                !values.contains(vn)
            },
            &format!("In constraint not satisfied. Expected one of [{expected}]"),
            maybe_path,
        )
    }
}
