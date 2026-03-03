use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::fmt::Display;

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MinExclusive {
    min_exclusive: ConcreteLiteral,
}

impl MinExclusive {
    pub fn new(literal: ConcreteLiteral) -> Self {
        MinExclusive { min_exclusive: literal }
    }

    pub fn min_exclusive(&self) -> &ConcreteLiteral {
        &self.min_exclusive
    }
}

impl Display for MinExclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinExclusive: {}", self.min_exclusive())
    }
}
