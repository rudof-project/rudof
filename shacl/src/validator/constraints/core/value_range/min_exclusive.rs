use std::fmt::{format, Debug};
use std::ops::Deref;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{NeighsRDF, RDFError, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use crate::ir::components::MinExclusive;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{validate_ask_with, validate_with, ConstraintError, NativeValidator, SparqlValidator};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::report::ValidationResult;
use crate::validator::nodes::ValueNodes;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MinExclusive {
    fn validate_native(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |n| {
                match S::term_as_sliteral(n) {
                    Ok(lit) => lit.partial_cmp(self.min_exclusive()).map(|o| o.is_le()).unwrap_or(true),
                    Err(_) => true
                }
            },
            &format!("MinExclusive({}) not satisfied", self.min_exclusive()),
            maybe_path
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MinExclusive {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &S, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let query_fn = |vn: &S::Term| {
            formatdoc! {
                " ASK {{ FILTER ({} < {}) }} ",
                vn, self.min_exclusive()
            }
        };

        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("MinExclusive({}) not satisfied", self.min_exclusive()),
            maybe_path
        )
    }
}