use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, Rdf, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, Literal};
use crate::ir::components::Datatype;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::iteration::ValueNodeIteration;
use crate::validation::report::ValidationResult;
use crate::validation::utils::validate_with;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Datatype  {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |vn| {
                if let Ok(lit) = S::term_as_literal(vn) {
                    match TryInto::<ConcreteLiteral>::try_into(lit.clone()) {
                        Ok(ConcreteLiteral::WrongDatatypeLiteral { .. }) => true,
                        Ok(_) => lit.datatype().get_iri().unwrap().as_str() != self.datatype().as_str(),
                        Err(_) => true,
                    }
                } else {
                    true
                }
            },
            &format!("Expected Datatype: {}", shapes_graph.prefix_map().qualify(self.datatype())),
            maybe_path
        )
    }
}
