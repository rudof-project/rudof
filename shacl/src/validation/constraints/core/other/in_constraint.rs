use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use crate::ir::components::In;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::iteration::ValueNodeIteration;
use crate::validation::report::ValidationResult;
use crate::validation::utils::validate_with;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for In {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |vn| {
                let values = self.values().iter().map(S::object_as_term).collect::<Vec<_>>();
                !values.contains(vn)
            },
            &format!("In constraint not satisfied. Expected one of {:?}", self.values()),
            maybe_path
        )
    }
}
