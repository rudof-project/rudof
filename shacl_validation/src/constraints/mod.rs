use core::{
    cardinality::{MaxCountConstraintComponent, MinCountConstraintComponent},
    logical::{
        AndConstraintComponent, NotConstraintComponent, OrConstraintComponent,
        XoneConstraintComponent,
    },
    other::{ClosedConstraintComponent, HasValueConstraintComponent, InConstraintComponent},
    property_pair::{
        DisjointConstraintComponent, EqualsConstraintComponent, LessThanConstraintComponent,
        LessThanOrEqualsConstraintComponent,
    },
    shape_based::{NodeConstraintComponent, QualifiedValueShapeConstraintComponent},
    string_based::{
        LanguageInConstraintComponent, MaxLengthConstraintComponent, MinLengthConstraintComponent,
        PatternConstraintComponent, UniqueLangConstraintComponent,
    },
    value::{ClassConstraintComponent, DatatypeConstraintComponent, NodeKindConstraintComponent},
    value_range::{
        MaxExclusiveConstraintComponent, MinExclusiveConstraintComponent,
        MinInclusiveConstraintComponent,
    },
};
use std::collections::HashSet;

use constraint_error::ConstraintError;
use shacl_ast::component::Component;
use srdf::{RDFNode, SRDFGraph};

use crate::validation_report::{report::ValidationReport, result::ValidationResult};

pub(crate) mod constraint_error;
pub mod core;

pub trait Evaluate {
    fn evaluate(
        &self,
        graph: &SRDFGraph,
        value_nodes: HashSet<RDFNode>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError>;

    fn make_validation_result(graph: &SRDFGraph, value_node: RDFNode) -> ValidationResult {
        ValidationResult::new(
            value_node,
            result_severity,
            result_path,
            source_constraint,
            source_constraint_component,
            source_shape,
            value,
        )
    }
}

pub struct ConstraintFactory;

impl ConstraintFactory {
    pub fn new_constraint(component: &Component) -> Box<dyn Evaluate> {
        match component {
            Component::Class(node) => Box::new(ClassConstraintComponent::new(node.to_owned())),
            Component::Datatype(iri_ref) => {
                Box::new(DatatypeConstraintComponent::new(iri_ref.to_owned()))
            }
            Component::NodeKind(node_kind) => {
                Box::new(NodeKindConstraintComponent::new(node_kind.to_owned()))
            }
            Component::MinCount(min_count) => {
                Box::new(MinCountConstraintComponent::new(min_count.to_owned()))
            }
            Component::MaxCount(max_count) => {
                Box::new(MaxCountConstraintComponent::new(max_count.to_owned()))
            }
            Component::MinExclusive(literal) => {
                Box::new(MinExclusiveConstraintComponent::new(literal.to_owned()))
            }
            Component::MaxExclusive(literal) => {
                Box::new(MaxExclusiveConstraintComponent::new(literal.to_owned()))
            }
            Component::MinInclusive(literal) => {
                Box::new(MinInclusiveConstraintComponent::new(literal.to_owned()))
            }
            Component::MaxInclusive(literal) => {
                Box::new(MaxExclusiveConstraintComponent::new(literal.to_owned()))
            }
            Component::MinLength(min_length) => {
                Box::new(MinLengthConstraintComponent::new(min_length.to_owned()))
            }
            Component::MaxLength(max_lenth) => {
                Box::new(MaxLengthConstraintComponent::new(max_lenth.to_owned()))
            }
            Component::Pattern { pattern, flags } => Box::new(PatternConstraintComponent::new(
                pattern.to_owned(),
                flags.to_owned(),
            )),
            Component::UniqueLang(unique_lang) => {
                Box::new(UniqueLangConstraintComponent::new(unique_lang.to_owned()))
            }
            Component::LanguageIn { langs } => {
                Box::new(LanguageInConstraintComponent::new(langs.to_owned()))
            }
            Component::Equals(iri_ref) => {
                Box::new(EqualsConstraintComponent::new(iri_ref.to_owned()))
            }
            Component::Disjoint(iri_ref) => {
                Box::new(DisjointConstraintComponent::new(iri_ref.to_owned()))
            }
            Component::LessThan(iri_ref) => {
                Box::new(LessThanConstraintComponent::new(iri_ref.to_owned()))
            }
            Component::LessThanOrEquals(iri_ref) => {
                Box::new(LessThanOrEqualsConstraintComponent::new(iri_ref.to_owned()))
            }
            Component::Or { shapes } => Box::new(OrConstraintComponent::new(shapes.to_owned())),
            Component::And { shapes } => Box::new(AndConstraintComponent::new(shapes.to_owned())),
            Component::Not { shape } => Box::new(NotConstraintComponent::new(shape.to_owned())),
            Component::Xone { shapes } => Box::new(XoneConstraintComponent::new(shapes.to_owned())),
            Component::Closed {
                is_closed,
                ignored_properties,
            } => Box::new(ClosedConstraintComponent::new(
                is_closed.to_owned(),
                ignored_properties.to_owned(),
            )),
            Component::Node { shape } => Box::new(NodeConstraintComponent::new(shape.to_owned())),
            Component::HasValue { value } => {
                Box::new(HasValueConstraintComponent::new(value.to_owned()))
            }
            Component::In { values } => Box::new(InConstraintComponent::new(values.to_owned())),
            Component::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => Box::new(QualifiedValueShapeConstraintComponent::new(
                shape.to_owned(),
                qualified_min_count.to_owned(),
                qualified_max_count.to_owned(),
                qualified_value_shapes_disjoint.to_owned(),
            )),
        }
    }
}
