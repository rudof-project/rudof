use indoc::formatdoc;
use shacl_ast::node_kind::NodeKind;
use srdf::{QuerySRDF, SRDF};
use std::sync::Arc;

use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

/// sh:nodeKind specifies a condition to be satisfied by the RDF node kind of
/// each value node.
///
/// https://www.w3.org/TR/shacl/#NodeKindConstraintComponent
pub(crate) struct Nodekind {
    node_kind: NodeKind,
}

impl Nodekind {
    pub fn new(node_kind: NodeKind) -> Self {
        Nodekind { node_kind }
    }
}

impl< S: SRDF> DefaultConstraintComponent< S> for Nodekind {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                let is_valid = match (
                    S::term_is_bnode(&value_node),
                    S::term_is_iri(&value_node),
                    S::term_is_literal(&value_node),
                ) {
                    (true, false, false) => matches!(
                        self.node_kind,
                        NodeKind::BlankNode
                            | NodeKind::BlankNodeOrIri
                            | NodeKind::BlankNodeOrLiteral
                    ),
                    (false, true, false) => matches!(
                        self.node_kind,
                        NodeKind::Iri | NodeKind::IRIOrLiteral | NodeKind::BlankNodeOrIri
                    ),
                    (false, false, true) => matches!(
                        self.node_kind,
                        NodeKind::Literal | NodeKind::IRIOrLiteral | NodeKind::BlankNodeOrLiteral
                    ),
                    _ => false,
                };

                if !is_valid {
                    let result = ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        Some(&value_node),
                    );
                    Some(result)
                } else {
                    None
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for Nodekind {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        let results = value_nodes
            .iter_full()
            .filter_map(move |(focus_node, value_node)| {
            let query = if S::term_is_iri(&value_node) {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                    ", self.node_kind
                }
            } else if S::term_is_bnode(&value_node) {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:Literal, sh:BlankNodeOrLiteral, sh:IRIOrLiteral ) ) }}
                    ", self.node_kind
                }
            } else {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:BlankNode, sh:BlankNodeOrIRI, sh:BlankNodeOrLiteral ) ) }}
                    ", self.node_kind
                }
            };

            let ask = match validation_context.store().query_ask(&query) {
                Ok(ask) => ask,
                Err(_) => return None,
            };

            if !ask {
                Some(ValidationResult::new(&focus_node, Arc::clone(&evaluation_context), Some(&value_node)))
            } else {
                None
            }
        });

        LazyValidationIterator::new(results)
    }
}
