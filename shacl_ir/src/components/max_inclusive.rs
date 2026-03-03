use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::fmt::Display;

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MaxInclusive {
    max_inclusive: ConcreteLiteral,
}

impl MaxInclusive {
    pub fn new(literal: ConcreteLiteral) -> Self {
        MaxInclusive { max_inclusive: literal }
    }

    pub fn max_inclusive(&self) -> &ConcreteLiteral {
        &self.max_inclusive
    }
}

impl Display for MaxInclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxInclusive: {}", self.max_inclusive())
    }
}
