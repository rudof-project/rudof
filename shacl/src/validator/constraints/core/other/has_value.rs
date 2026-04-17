use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use crate::ir::components::HasValue;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{validate_with, ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validator::engine::{Engine, SparqlEngine};
use crate::validator::iteration::FocusNodeIteration;
use crate::validator::report::ValidationResult;
use crate::validator::nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for HasValue {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
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
            maybe_path
        )
    }
}
