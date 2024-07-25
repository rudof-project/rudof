use std::collections::HashSet;

use oxigraph::{model::Term, store::Store};
use prefixmap::IriRef;
use shacl_ast::node_kind::NodeKind;
use srdf::RDFNode;

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    validation_report::report::ValidationReport,
};

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct ClassConstraintComponent {
    class_rule: RDFNode,
}

impl ClassConstraintComponent {
    pub fn new(class_rule: RDFNode) -> Self {
        ClassConstraintComponent { class_rule }
    }
}

impl Evaluate for ClassConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Ok(())
    }
}

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct DatatypeConstraintComponent {
    iri_ref: IriRef,
}

impl DatatypeConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        DatatypeConstraintComponent { iri_ref }
    }
}

impl Evaluate for DatatypeConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

/// sh:nodeKind specifies a condition to be satisfied by the RDF node kind of
/// each value node.
///
/// https://www.w3.org/TR/shacl/#NodeKindConstraintComponent
pub(crate) struct NodeKindConstraintComponent {
    node_kind: NodeKind,
}

impl NodeKindConstraintComponent {
    pub fn new(node_kind: NodeKind) -> Self {
        NodeKindConstraintComponent { node_kind }
    }
}

impl Evaluate for NodeKindConstraintComponent {
    fn evaluate(
        &self,
        store: &Store,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}
