use std::ops::Not;

use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shacl_engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF, term::Term};
use shacl::ir::components::Nodekind as NodeKindComponent;
use shacl::ir::{IRComponent, IRSchema, IRShape};
use shacl::types::NodeKind;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for NodeKindComponent {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        _: &S,
        _engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let node_kind = |value_node: &S::Term| {
            match (value_node.is_blank_node(), value_node.is_iri(), value_node.is_literal()) {
                (true, false, false) => matches!(
                    self.node_kind(),
                    NodeKind::BNode | NodeKind::BNodeOrIri | NodeKind::BNodeOrLit
                ),
                (false, true, false) => matches!(
                    self.node_kind(),
                    NodeKind::Iri | NodeKind::IriOrLit | NodeKind::BNodeOrIri
                ),
                (false, false, true) => matches!(
                    self.node_kind(),
                    NodeKind::Lit | NodeKind::IriOrLit | NodeKind::BNodeOrLit
                ),
                _ => false,
            }
            .not()
        };

        let message = format!(
            "Nodekind constraint not satisfied. Expected node kind: {}",
            self.node_kind()
        );
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            node_kind,
            &message,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for NodeKindComponent {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let node_kind = self.node_kind().clone();

        let query = move |value_node: &S::Term| {
            if value_node.is_iri() {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                    ",node_kind
                }
            } else if value_node.is_blank_node() {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:Literal, sh:BlankNodeOrLiteral, sh:IRIOrLiteral ) ) }}
                    ", node_kind
                }
            } else {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:BlankNode, sh:BlankNodeOrIRI, sh:BlankNodeOrLiteral ) ) }}
                    ", node_kind
                }
            }
        };

        let message = format!(
            "Nodekind constraint not satisfied. Expected node kind: {}",
            self.node_kind()
        );
        validate_ask_with(component, shape, store, value_nodes, query, &message, maybe_path)
    }
}
