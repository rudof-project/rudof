use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use crate::ir::components::MinCount;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::iteration::FocusNodeIteration;
use crate::validation::report::ValidationResult;
use crate::validation::utils::validate_with;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for MinCount {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        if self.min_count() == 0 { return Ok(Default::default()); }

        validate_with(
            component,
            shape,
            value_nodes,
            FocusNodeIteration,
            |t| t.len() < self.min_count(),
            &format!("MinCount({}) not satisfied", self.min_count()),
            maybe_path
        )
    }
}
