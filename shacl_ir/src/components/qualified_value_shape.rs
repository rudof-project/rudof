use crate::shape_label_idx::ShapeLabelIdx;
use std::fmt::Display;

/// QualifiedValueShape Constraint Component.
///
/// sh:qualifiedValueShape specifies the condition that a specified number of
///  value nodes conforms to the given shape. Each sh:qualifiedValueShape can
///  have: one value for sh:qualifiedMinCount, one value for
///  sh:qualifiedMaxCount or, one value for each, at the same subject.
///
/// https://www.w3.org/TR/shacl/#QualifiedValueShapeConstraintComponent
#[derive(Debug, Clone)]
pub struct QualifiedValueShape {
    shape: ShapeLabelIdx,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    qualified_value_shapes_disjoint: Option<bool>,
    siblings: Vec<ShapeLabelIdx>,
}

impl QualifiedValueShape {
    pub fn new(
        shape: ShapeLabelIdx,
        qualified_min_count: Option<isize>,
        qualified_max_count: Option<isize>,
        qualified_value_shapes_disjoint: Option<bool>,
        siblings: Vec<ShapeLabelIdx>,
    ) -> Self {
        QualifiedValueShape {
            shape,
            qualified_min_count,
            qualified_max_count,
            qualified_value_shapes_disjoint,
            siblings,
        }
    }

    pub fn shape(&self) -> &ShapeLabelIdx {
        &self.shape
    }

    pub fn qualified_min_count(&self) -> Option<isize> {
        self.qualified_min_count
    }

    pub fn qualified_max_count(&self) -> Option<isize> {
        self.qualified_max_count
    }

    pub fn siblings(&self) -> &Vec<ShapeLabelIdx> {
        &self.siblings
    }

    pub fn qualified_value_shapes_disjoint(&self) -> Option<bool> {
        self.qualified_value_shapes_disjoint
    }
}

impl Display for QualifiedValueShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "QualifiedValueShape: shape: {}, qualifiedMinCount: {:?}, qualifiedMaxCount: {:?}, qualifiedValueShapesDisjoint: {:?}{}",
            self.shape(),
            self.qualified_min_count(),
            self.qualified_max_count(),
            self.qualified_value_shapes_disjoint(),
            if self.siblings().is_empty() {
                "".to_string()
            } else {
                format!(
                    ", siblings: [{}]",
                    self.siblings()
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        )
    }
}
