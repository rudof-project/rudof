use crate::error::ValidationError;
use crate::ir::components::MaxLength;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{validate_with, NativeValidator};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::literal::Literal;
use rudof_rdf::rdf_core::term::{Iri, Term};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

#[cfg(feature = "sparql")]
use crate::validator::constraints::{term_as_sparql, validate_ask_with_opt, BasicSparqlValidator};
#[cfg(feature = "sparql")]
use indoc::formatdoc;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::query::QueryRDF;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MaxLength {
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
        let max_length_fn = |vn: &S::Term| {
            if vn.is_blank_node() {
                return true;
            }
            if let Ok(iri) = S::term_as_iri(vn) {
                return iri.as_str().len() > self.max_length() as usize;
            }
            if let Ok(lit) = S::term_as_literal(vn) {
                return lit.lexical_form().len() > self.max_length() as usize;
            }
            true
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            max_length_fn,
            &format!("MaxLength({}) not satisfied", self.max_length()),
            maybe_path,
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for MaxLength {
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
        let max = self.max_length();

        let query_fn = |vn: &S::Term| -> Option<String> {
            if vn.is_blank_node() {
                return Some("ASK { FILTER(false) }".to_string());
            }
            let vn_sparql = term_as_sparql::<S>(vn)?;
            Some(formatdoc! {"
                ASK {{ FILTER(STRLEN(STR({vn_sparql})) <= {max}) }}
            "})
        };

        validate_ask_with_opt(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("MaxLength({max}) not satisfied"),
            maybe_path,
        )
    }
}
