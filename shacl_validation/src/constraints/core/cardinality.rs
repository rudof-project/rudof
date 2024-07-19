use crate::{constraints::Evaluate, validation_report::ValidationResult};

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// https://www.w3.org/TR/shacl/#MinCountConstraintComponent
pub(crate) struct MinCountConstraintComponent {
    min_count: isize,
}

impl MinCountConstraintComponent {
    pub fn new(min_count: isize) -> Self {
        MinCountConstraintComponent { min_count }
    }
}

impl Evaluate for MinCountConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}

/// sh:maxCount specifies the maximum number of value nodes that satisfy the
/// condition.
///
/// https://www.w3.org/TR/shacl/#MaxCountConstraintComponent
pub(crate) struct MaxCountConstraintComponent {
    max_count: isize,
}

impl MaxCountConstraintComponent {
    pub fn new(max_count: isize) -> Self {
        MaxCountConstraintComponent { max_count }
    }
}

impl Evaluate for MaxCountConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}
