use std::fmt::Display;

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// - IRI: https://www.w3.org/TR/shacl/#MinCountConstraintComponent
/// - DEF: If the number of value nodes is less than $minCount, there is a
///   validation result.
#[derive(Debug, Clone)]
pub struct MinCount {
    min_count: usize,
}

impl MinCount {
    pub fn new(min_count: isize) -> Self {
        MinCount {
            min_count: min_count as usize,
        }
    }

    pub fn min_count(&self) -> usize {
        self.min_count
    }
}

impl Display for MinCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinCount: {}", self.min_count())
    }
}
