use std::ops::Not;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Nodekind;
use shacl_ast::compiled::shape::CompiledShape;
use shacl_ast::node_kind::NodeKind;
use srdf::Query;
use srdf::Sparql;
use std::fmt::Debug;

impl<S: Query + Debug + 'static> NativeValidator<S> for Nodekind {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let node_kind = |value_node: &S::Term| {
            match (
                S::term_is_bnode(value_node),
                S::term_is_iri(value_node),
                S::term_is_literal(value_node),
            ) {
                (true, false, false) => matches!(
                    self.node_kind(),
                    NodeKind::BlankNode | NodeKind::BlankNodeOrIri | NodeKind::BlankNodeOrLiteral
                ),
                (false, true, false) => matches!(
                    self.node_kind(),
                    NodeKind::Iri | NodeKind::IRIOrLiteral | NodeKind::BlankNodeOrIri
                ),
                (false, false, true) => matches!(
                    self.node_kind(),
                    NodeKind::Literal | NodeKind::IRIOrLiteral | NodeKind::BlankNodeOrLiteral
                ),
                _ => false,
            }
            .not()
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, node_kind)
    }
}

impl<S: Sparql + Debug + 'static> SparqlValidator<S> for Nodekind {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let node_kind = self.node_kind().clone();

        let query = move |value_node: &S::Term| {
            if S::term_is_iri(value_node) {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                    ",node_kind
                }
            } else if S::term_is_bnode(value_node) {
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

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
