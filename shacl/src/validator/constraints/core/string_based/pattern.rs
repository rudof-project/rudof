use crate::error::ValidationError;
use crate::ir::components::Pattern;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{NativeValidator, escape_sparql_string, validate_with};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::Term;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

#[cfg(feature = "sparql")]
use crate::validator::constraints::{BasicSparqlValidator, term_as_sparql, validate_ask_with_opt};
#[cfg(feature = "sparql")]
use indoc::formatdoc;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::query::QueryRDF;

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
    ) -> Result<Vec<ValidationResult>, ValidationError> {
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
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for Pattern {
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
        let pattern = escape_sparql_string(self.pattern());
        let flags_arg = self
            .flags()
            .map(|f| format!(", \"{}\"", escape_sparql_string(f)))
            .unwrap_or_default();

        let query_fn = |vn: &S::Term| -> Option<String> {
            if vn.is_blank_node() {
                return Some("ASK { FILTER(false) }".to_string());
            }
            let vn_sparql = term_as_sparql::<S>(vn)?;
            Some(formatdoc! {"
                ASK {{ FILTER(REGEX(STR({vn_sparql}), \"{pattern}\"{flags_arg})) }}
            "})
        };

        validate_ask_with_opt(
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
