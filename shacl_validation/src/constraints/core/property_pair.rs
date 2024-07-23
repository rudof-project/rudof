use std::collections::HashSet;

use oxrdf::Term;
use prefixmap::IriRef;
use srdf::SRDFGraph;

use crate::{constraints::Evaluate, validation_report::result::ValidationResult};

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
pub(crate) struct EqualsConstraintComponent {
    iri_ref: IriRef,
}

impl EqualsConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        EqualsConstraintComponent { iri_ref }
    }
}

impl Evaluate for EqualsConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, value_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
pub(crate) struct DisjointConstraintComponent {
    iri_ref: IriRef,
}

impl DisjointConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        DisjointConstraintComponent { iri_ref }
    }
}

impl Evaluate for DisjointConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, value_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
pub(crate) struct LessThanConstraintComponent {
    iri_ref: IriRef,
}

impl LessThanConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        LessThanConstraintComponent { iri_ref }
    }
}

impl Evaluate for LessThanConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, value_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
pub(crate) struct LessThanOrEqualsConstraintComponent {
    iri_ref: IriRef,
}

impl LessThanOrEqualsConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        LessThanOrEqualsConstraintComponent { iri_ref }
    }
}

impl Evaluate for LessThanOrEqualsConstraintComponent {
    fn evaluate(&self, graph: &SRDFGraph, value_nodes: HashSet<Term>) -> Option<ValidationResult> {
        todo!()
    }
}
