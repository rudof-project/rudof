use crate::ir::components::Datatype;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{ConstraintError, Validator, validate_with};
use crate::validator::engine::{Engine};
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, Literal};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Datatype {
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
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
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
            &format!(
                "Expected Datatype: {}",
                shapes_graph.prefix_map().qualify(self.datatype())
            ),
            maybe_path,
        )
    }
}
