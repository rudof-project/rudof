use std::fmt::Debug;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Term;
use crate::ir::components::Pattern;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{ConstraintError, NativeValidator, SparqlValidator};
use crate::validation::engine::Engine;
use crate::validation::iteration::ValueNodeIteration;
use crate::validation::report::ValidationResult;
use crate::validation::utils::{validate_ask_with, validate_with};
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Pattern {
    fn validate_native(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |vn| {
                if vn.is_blank_node() {
                    true
                } else {
                    !self.match_str(vn.lexical_form().as_str())
                }
            },
            &format!("Pattern({}) not satisfied", self.pattern()),
            maybe_path
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for Pattern {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &S, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let query_fn =  |vn: &S::Term| match self.flags() {
            None => formatdoc! {
                "ASK {{ FILTER (regex(str({}), {})) }}",
                vn, self.pattern()
            },
            Some(flags) => formatdoc! {
                "ASK {{ FILTER (regex(str({}), {}, {})) }}",
                vn, self.pattern(), flags
            }
        };

        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("Pattern({}) not satisfied", self.pattern()),
            maybe_path
        )
    }
}