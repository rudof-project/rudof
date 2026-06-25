use crate::error::ValidationError;
use crate::ir::components::MaxExclusive;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{NativeValidator, validate_with};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

#[cfg(feature = "sparql")]
use crate::validator::constraints::{BasicSparqlValidator, object_as_sparql, term_as_sparql, validate_ask_with_opt};
#[cfg(feature = "sparql")]
use indoc::formatdoc;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::query::QueryRDF;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::term::{Object, Term};

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MaxExclusive {
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
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |n| match S::term_as_sliteral(n) {
                Ok(lit) => lit.partial_cmp(self.max_exclusive()).map(|o| o.is_ge()).unwrap_or(true),
                Err(_) => true,
            },
            &format!("MaxExclusive({}) not satisfied", self.max_exclusive()),
            maybe_path,
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for MaxExclusive {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        let threshold = match object_as_sparql(&Object::literal(self.max_exclusive().clone())) {
            Some(s) => s,
            None => return Ok(Vec::new()),
        };

        let query_fn = |vn: &S::Term| -> Option<String> {
            if !vn.is_literal() {
                return Some("ASK { FILTER(false) }".to_string());
            }
            let vn_sparql = term_as_sparql::<S>(vn)?;
            Some(formatdoc! {"
                ASK {{ FILTER(COALESCE({vn_sparql} < {threshold}, false)) }}
            "})
        };

        validate_ask_with_opt(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("MaxExclusive({}) not satisfied", self.max_exclusive()),
            maybe_path,
        )
    }
}
