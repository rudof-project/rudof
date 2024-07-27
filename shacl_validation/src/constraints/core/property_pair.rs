use std::collections::HashSet;

use oxigraph::{model::Term, store::Store};
use prefixmap::IriRef;

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    validation_report::report::ValidationReport,
};

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct EqualsConstraintComponent {
    iri_ref: IriRef,
}

impl EqualsConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        EqualsConstraintComponent { iri_ref }
    }
}

impl Evaluate for EqualsConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct DisjointConstraintComponent {
    iri_ref: IriRef,
}

impl DisjointConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        DisjointConstraintComponent { iri_ref }
    }
}

impl Evaluate for DisjointConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct LessThanConstraintComponent {
    iri_ref: IriRef,
}

impl LessThanConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        LessThanConstraintComponent { iri_ref }
    }
}

impl Evaluate for LessThanConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct LessThanOrEqualsConstraintComponent {
    iri_ref: IriRef,
}

impl LessThanOrEqualsConstraintComponent {
    pub fn new(iri_ref: IriRef) -> Self {
        LessThanOrEqualsConstraintComponent { iri_ref }
    }
}

impl Evaluate for LessThanOrEqualsConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}
