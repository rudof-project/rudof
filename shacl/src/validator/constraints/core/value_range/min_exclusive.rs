use crate::ir::components::MinExclusive;
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
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MinExclusive {
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
            |n| match S::term_as_sliteral(n) {
                Ok(lit) => lit.partial_cmp(self.min_exclusive()).map(|o| o.is_le()).unwrap_or(true),
                Err(_) => true,
            },
            &format!("MinExclusive({}) not satisfied", self.min_exclusive()),
            maybe_path,
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MinExclusive {
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
            maybe_path,
        )
    }
}
