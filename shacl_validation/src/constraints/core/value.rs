use std::collections::HashSet;

use oxrdf::Term;
use prefixmap::IriRef;
use shacl_ast::node_kind::NodeKind;
use srdf::{RDFNode, SRDFGraph};

use crate::{constraints::Evaluate, validation_report::result::ValidationResult};

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct ClassConstraintComponent {
    node: RDFNode,
}

impl ClassConstraintComponent {
    pub fn new(node: RDFNode) -> Self {
        ClassConstraintComponent { node }
    }
}

impl Evaluate for ClassConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, focus_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
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
    fn evaluate(&self, graph: &SRDFGraph, focus_nodes: HashSet<Term>) -> Option<ValidationResult> {
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
    fn evaluate(&self, graph: &SRDFGraph, focus_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}
