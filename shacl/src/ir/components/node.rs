use crate::ir::shape_label_idx::ShapeLabelIdx;
use std::fmt::{Display, Formatter};

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
#[derive(Debug, Clone)]
pub(crate) struct Node {
    shape: ShapeLabelIdx,
}

impl Node {
    pub fn new(shape: ShapeLabelIdx) -> Self {
        Node { shape }
    }

    pub fn shape(&self) -> &ShapeLabelIdx {
        &self.shape
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node [{}]", self.shape())
    }
}
