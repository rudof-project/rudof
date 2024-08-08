use std::collections::HashSet;

use indoc::formatdoc;
use shacl_ast::node_kind::NodeKind;
use shacl_ast::Schema;
use srdf::{QuerySRDF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
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
        _store: &S,
        _: &Schema,
        _: &DefaultValidatorRunner,
        _value_nodes: &HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Nodekind {
    fn evaluate_sparql(
        &self,
        store: &S,
        _: &Schema,
        _: &SparqlValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for node in value_nodes {
            let query = if S::term_is_iri(node) {
                formatdoc! {"
                    PREFIX sh: <http://www.w3.org/ns/shacl#>
                    ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                ", self.node_kind
                }
            } else if S::term_is_bnode(node) {
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
            let ask = match store.query_ask(&query) {
                Ok(ask) => ask,
                Err(_) => return Err(ConstraintError::Query),
            };
            if !ask {
                ans = false;
                report.make_validation_result(Some(node));
            }
        }
        Ok(ans)
    }
}
