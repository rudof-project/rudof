use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::fmt::{Display, Formatter};

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
#[derive(Debug, Clone)]
pub(crate) struct MinInclusive {
    min_inclusive: ConcreteLiteral,
}

impl MinInclusive {
    pub fn new(literal: ConcreteLiteral) -> Self {
        MinInclusive { min_inclusive: literal }
    }

    pub fn min_inclusive(&self) -> &ConcreteLiteral {
        &self.min_inclusive
    }
}

impl Display for MinInclusive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinInclusive: {}", self.min_inclusive())
    }
}
