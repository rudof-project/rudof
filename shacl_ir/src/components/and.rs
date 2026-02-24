use crate::shape_label_idx::ShapeLabelIdx;
use std::fmt::Display;

/// sh:and specifies the condition that each value node conforms to all provided
/// shapes. This is comparable to conjunction and the logical "and" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
#[derive(Debug, Clone)]
pub struct And {
    shapes: Vec<ShapeLabelIdx>,
}

impl And {
    pub fn new(shapes: Vec<ShapeLabelIdx>) -> Self {
        And { shapes }
    }

    pub fn shapes(&self) -> &Vec<ShapeLabelIdx> {
        &self.shapes
    }
}

impl Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "And [{}]",
            self.shapes()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
