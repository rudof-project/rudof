use crate::shape_label_idx::ShapeLabelIdx;
use std::fmt::Display;

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent

#[derive(Debug, Clone)]
pub struct Or {
    shapes: Vec<ShapeLabelIdx>,
}

impl Or {
    pub fn new(shapes: Vec<ShapeLabelIdx>) -> Self {
        Or { shapes }
    }

    pub fn shapes(&self) -> &Vec<ShapeLabelIdx> {
        &self.shapes
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Or[{}]",
            self.shapes()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
