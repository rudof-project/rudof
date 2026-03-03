use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::fmt::Display;

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MinInclusive {
    min_inclusive: ConcreteLiteral,
}

impl MinInclusive {
    pub fn new(literal: ConcreteLiteral) -> Self {
        MinInclusive { min_inclusive: literal }
    }

    pub fn min_inclusive_value(&self) -> &ConcreteLiteral {
        &self.min_inclusive
    }
}

impl Display for MinInclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinInclusive: {}", self.min_inclusive)
    }
}
