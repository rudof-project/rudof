use indoc::formatdoc;
use shacl_ast::node_kind::NodeKind;
use srdf::{QuerySRDF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::shape::ValueNode;
use crate::validation_report::report::ValidationReport;

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

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Nodekind {
    fn evaluate_default(
        &self,
        _: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                let is_valid = match (
                    S::term_is_bnode(value_node),
                    S::term_is_iri(value_node),
                    S::term_is_literal(value_node),
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
                    ans = false;
                    report.make_validation_result(focus_node, context, Some(value_node));
                }
            }
        }
        Ok(ans)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Nodekind {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                let query = if S::term_is_iri(value_node) {
                    formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                    ", self.node_kind
                    }
                } else if S::term_is_bnode(value_node) {
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
                let ask = match executor.store().query_ask(&query) {
                    Ok(ask) => ask,
                    Err(_) => return Err(ConstraintError::Query),
                };
                if !ask {
                    ans = false;
                    report.make_validation_result(focus_node, context, Some(value_node));
                }
            }
        }
        Ok(ans)
    }
}
