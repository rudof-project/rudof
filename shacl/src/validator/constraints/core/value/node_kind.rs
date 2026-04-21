use std::fmt::Debug;
use std::ops::Not;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Term;
use crate::ir::components::Nodekind;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::NodeKind;
use crate::validator::constraints::{validate_ask_with, validate_with, ConstraintError, NativeValidator, SparqlValidator};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::report::ValidationResult;
use crate::validator::nodes::ValueNodes;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Nodekind {
    fn validate_native(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
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
                _ => false
            }.not()
        };

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            nk_fn,
            &format!("NodeKind constraint not satisfied. Expected node kind: {}", self.node_kind()),
            maybe_path
        )
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for Nodekind {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &S, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let query_fn = |vn: &S::Term| {
            if vn.is_iri() {
                formatdoc! {"
                    PREFIX sh: <http://www.w3.org/ns/shacl#>
                    ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                ", self.node_kind()
                }
            } else if vn.is_literal() {
                formatdoc! {"
                    PREFIX sh: <http://www.w3.org/ns/shacl#>
                    ASK {{ FILTER ({} IN ( sh:Literal, sh:BlankNodeOrLiteral, sh:IRIOrLiteral ) ) }}
                ", self.node_kind()
                }
            } else {
                formatdoc! {"
                    PREFIX sh: <http://www.w3.org/ns/shacl#>
                    ASK {{ FILTER ({} IN ( sh:BlankNode, sh:BlankNodeOrIRI, sh:BlankNodeOrLiteral ) ) }}
                ", self.node_kind()
                }
            }
        };

        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query_fn,
            &format!("NodeKind constraint not satisfied. Expected node kind: {}", self.node_kind()),
            maybe_path
        )
    }
}