use crate::shape_label_idx::ShapeLabelIdx;
use std::fmt::Display;

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[derive(Debug, Clone)]
pub struct Not {
    shape: ShapeLabelIdx,
}

impl Not {
    pub fn new(shape: ShapeLabelIdx) -> Self {
        Not { shape }
    }

    pub fn shape(&self) -> &ShapeLabelIdx {
        &self.shape
    }
}

impl Display for Not {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not [{}]", self.shape)
    }
}
