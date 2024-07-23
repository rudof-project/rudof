use std::collections::HashSet;

use oxrdf::Term;
use srdf::{RDFNode, SRDFGraph};

use crate::{constraints::Evaluate, validation_report::result::ValidationResult};

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
pub(crate) struct NotConstraintComponent {
    shape: RDFNode,
}

impl NotConstraintComponent {
    pub fn new(shape: RDFNode) -> Self {
        NotConstraintComponent { shape }
    }
}

impl Evaluate for NotConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, value_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
pub(crate) struct AndConstraintComponent {
    shapes: Vec<RDFNode>,
}

impl AndConstraintComponent {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        AndConstraintComponent { shapes }
    }
}

impl Evaluate for AndConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, value_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
pub(crate) struct OrConstraintComponent {
    shapes: Vec<RDFNode>,
}

impl OrConstraintComponent {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        OrConstraintComponent { shapes }
    }
}

impl Evaluate for OrConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, value_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
pub(crate) struct XoneConstraintComponent {
    shapes: Vec<RDFNode>,
}

impl XoneConstraintComponent {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        XoneConstraintComponent { shapes }
    }
}

impl Evaluate for XoneConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, value_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}
