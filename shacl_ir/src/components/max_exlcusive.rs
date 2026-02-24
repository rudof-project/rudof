use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::fmt::Display;

/// https://www.w3.org/TR/shacl/#MaxExclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MaxExclusive {
    max_exclusive: ConcreteLiteral,
}

impl MaxExclusive {
    pub fn new(literal: ConcreteLiteral) -> Self {
        MaxExclusive { max_exclusive: literal }
    }

    pub fn max_exclusive(&self) -> &ConcreteLiteral {
        &self.max_exclusive
    }
}

impl Display for MaxExclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxExclusive: {}", self.max_exclusive())
    }
}
