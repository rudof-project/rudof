use std::collections::HashSet;

use indoc::formatdoc;
use shacl_ast::node_kind::NodeKind;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::oxigraph::ask;
use crate::helper::term::Term;
use crate::runner::oxigraph::OxigraphStore;
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

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Nodekind {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<'a> ConstraintComponent<OxigraphStore<'a>> for Nodekind {
    fn evaluate(
        &self,
        store: &OxigraphStore<'a>,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            let query = match node {
                Term::IRI(_) => Some(formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                    ", self.node_kind
                }),
                Term::BlankNode(_) => Some(formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:Literal, sh:BlankNodeOrLiteral, sh:IRIOrLiteral ) ) }}
                    ", self.node_kind
                }),
                Term::Literal(_) => Some(formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:BlankNode, sh:BlankNodeOrIRI, sh:BlankNodeOrLiteral ) ) }}
                    ", self.node_kind
                }),
            };
            match query {
                Some(query) => {
                    if !ask(store, query)? {
                        <Nodekind as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                            self,
                            Some(node),
                            report,
                        );
                    }
                }
                None => {
                    <Nodekind as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                        self,
                        Some(node),
                        report,
                    )
                }
            }
        }
        Ok(())
    }
}
