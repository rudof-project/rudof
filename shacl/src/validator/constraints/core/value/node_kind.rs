use crate::error::ValidationError;
use crate::ir::components::Nodekind;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::NodeKind;
use crate::validator::constraints::{validate_with, NativeValidator};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::Term;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;
use std::ops::Not;

#[cfg(feature = "sparql")]
use crate::validator::constraints::{term_as_sparql, validate_ask_with_opt, BasicSparqlValidator};
#[cfg(feature = "sparql")]
use indoc::formatdoc;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::query::QueryRDF;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Nodekind {
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
        let nk_fn = |vn: &S::Term| {
            match (vn.is_blank_node(), vn.is_iri(), vn.is_literal()) {
                (true, false, false) => matches!(
                    self.node_kind(),
                    NodeKind::BNode | NodeKind::BNodeOrIri | NodeKind::BNodeOrLit
                ),
                (false, true, false) => matches!(
                    self.node_kind(),
                    NodeKind::Iri | NodeKind::BNodeOrIri | NodeKind::IriOrLit
                ),
                (false, false, true) => matches!(
                    self.node_kind(),
                    NodeKind::Lit | NodeKind::IriOrLit | NodeKind::BNodeOrLit
                ),
                _ => false,
            }
            .not()
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            nk_fn,
            &format!(
                "NodeKind constraint not satisfied. Expected node kind: {}",
                self.node_kind()
            ),
            maybe_path,
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for Nodekind {
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
        let allowed_iri = matches!(
            self.node_kind(),
            NodeKind::Iri | NodeKind::BNodeOrIri | NodeKind::IriOrLit
        );
        let allowed_bnode = matches!(
            self.node_kind(),
            NodeKind::BNode | NodeKind::BNodeOrIri | NodeKind::BNodeOrLit
        );
        let allowed_lit = matches!(
            self.node_kind(),
            NodeKind::Lit | NodeKind::IriOrLit | NodeKind::BNodeOrLit
        );

        let query_fn = |vn: &S::Term| -> Option<String> {
            if vn.is_blank_node() {
                let asserted = if allowed_bnode { "true" } else { "false" };
                return Some(format!("ASK {{ FILTER({asserted}) }}"));
            }
            let vn_sparql = term_as_sparql::<S>(vn)?;
            let test = if vn.is_iri() {
                if allowed_iri {
                    format!("isIRI({vn_sparql})")
                } else {
                    "false".to_string()
                }
            } else if vn.is_literal() {
                if allowed_lit {
                    format!("isLiteral({vn_sparql})")
                } else {
                    "false".to_string()
                }
            } else {
                "false".to_string()
            };
            Some(formatdoc! {"
                ASK {{ FILTER({test}) }}
            "})
        };

        validate_ask_with_opt(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!(
                "NodeKind constraint not satisfied. Expected node kind: {}",
                self.node_kind()
            ),
            maybe_path,
        )
    }
}
