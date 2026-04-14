use std::fmt::Debug;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{NeighsRDF, RDFError, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use crate::ir::components::MaxInclusive;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{ConstraintError, NativeValidator, SparqlValidator};
use crate::validation::engine::Engine;
use crate::validation::iteration::ValueNodeIteration;
use crate::validation::report::ValidationResult;
use crate::validation::utils::{validate_ask_with, validate_with};
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MaxInclusive {
    fn validate_native(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |n| {
                match S::term_as_sliteral(n) {
                    Ok(lit) => lit.partial_cmp(self.max_inclusive()).map(|o| o.is_gt()).unwrap_or(true),
                    Err(_) => true,
                }
            },
            &format!("MaxInclusive({}) not satisfied", self.max_inclusive()),
            maybe_path
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MaxInclusive {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &S, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let query_fn = |vn: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} >= {}) }} ",
                vn, self.max_inclusive()
            }
        };

        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("MaxInclusive({}) not satisfied", self.max_inclusive()),
            maybe_path
        )
    }
}