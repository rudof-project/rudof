use crate::ir::components::Pattern;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{
    ConstraintError, NativeValidator, SparqlValidator, validate_ask_with, validate_with,
};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use indoc::formatdoc;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Term;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Pattern {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        _: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
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
            maybe_path,
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for Pattern {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let query_fn = |vn: &S::Term| match self.flags() {
            None => formatdoc! {
                "ASK {{ FILTER (regex(str({}), {})) }}",
                vn, self.pattern()
            },
            Some(flags) => formatdoc! {
                "ASK {{ FILTER (regex(str({}), {}, {})) }}",
                vn, self.pattern(), flags
            },
        };

        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("Pattern({}) not satisfied", self.pattern()),
            maybe_path,
        )
    }
}
