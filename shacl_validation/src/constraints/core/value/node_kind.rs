use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::Nodekind;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;
use srdf::Term;

use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;

impl<Q: Query> Validator<Q, NativeEngine> for Nodekind {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        _store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let node_kind = |value_node: &Q::Term| Ok(self.node_kind() != &value_node.kind());
        validate_with(component, shape, value_nodes, ValueNodeIteration, node_kind)
    }
}

impl<S: Sparql + Query> Validator<S, SparqlEngine> for Nodekind {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
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

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
