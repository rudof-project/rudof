use srdf::RDFNode;

use crate::{constraints::Evaluate, validation_report::ValidationResult};

// TODO: missing PropertyConstraintComponent

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
pub(crate) struct NodeConstraintComponent {
    shape: RDFNode,
}

impl NodeConstraintComponent {
    pub fn new(shape: RDFNode) -> Self {
        NodeConstraintComponent { shape }
    }
}

impl Evaluate for NodeConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:qualifiedValueShape specifies the condition that a specified number of
///  value nodes conforms to the given shape. Each sh:qualifiedValueShape can
///  have: one value for sh:qualifiedMinCount, one value for
///  sh:qualifiedMaxCount or, one value for each, at the same subject.
///
/// https://www.w3.org/TR/shacl/#QualifiedValueShapeConstraintComponent
pub(crate) struct QualifiedValueShapeConstraintComponent {
    shape: RDFNode,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    qualified_value_shapes_disjoint: Option<bool>,
}

impl QualifiedValueShapeConstraintComponent {
    pub fn new(
        shape: RDFNode,
        qualified_min_count: Option<isize>,
        qualified_max_count: Option<isize>,
        qualified_value_shapes_disjoint: Option<bool>,
    ) -> Self {
        QualifiedValueShapeConstraintComponent {
            shape,
            qualified_min_count,
            qualified_max_count,
            qualified_value_shapes_disjoint,
        }
    }
}

impl Evaluate for QualifiedValueShapeConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}
