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
use shacl_ast::node_kind::NodeKind;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::Nodekind;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use srdf::Term;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Nodekind {
    fn validate_native(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        _: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let node_kind = |value_node: &S::Term| {
            match (
                value_node.is_blank_node(),
                value_node.is_iri(),
                value_node.is_literal(),
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

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for Nodekind {
    fn validate_sparql(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
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
        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query,
            &message,
            maybe_path,
        )
    }
}
