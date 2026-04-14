use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use crate::ir::components::MaxCount;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::error::ConstraintError;
use crate::validation::constraints::{NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::iteration::FocusNodeIteration;
use crate::validation::report::ValidationResult;
use crate::validation::utils::validate_with;
use crate::validation::value_nodes::ValueNodes;

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
