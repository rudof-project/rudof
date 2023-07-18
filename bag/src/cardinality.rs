use crate::Max;
use std::cmp;
use std::fmt;

pub type Min = usize;

#[derive(PartialEq, Eq, Clone)]
pub struct Cardinality {
    pub min: Min,
    pub max: Max,
}

impl Cardinality {
    pub fn from(min: usize, max: Max) -> Cardinality {
        Cardinality { min, max }
    }

    pub fn nullable(&self) -> bool {
        self.min == 0
    }

    pub fn contains(&self, n: usize) -> bool {
        n >= self.min && self.max.greater_or_equal(n)
    }

    pub fn minus(&self, n: usize) -> Option<Cardinality> {
        if self.contains(n) {
            let min = if self.min > n { self.min - n } else { 0 };
            Some(Cardinality {
                min: cmp::max(min, 0),
                max: self.max.minus(n),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Cardinality {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match (&self.min, &self.max) {
            (0, Max::IntMax(1)) => write!(dest, "?"),
            (0, Max::Unbounded) => write!(dest, "*"),
            (1, Max::Unbounded) => write!(dest, "+"),
            (min, Max::Unbounded) => write!(dest, "{{{},}}", min),
            (min, max) => write!(dest, "{{{}, {}}}", min, max),
        }
    }
}

impl fmt::Debug for Cardinality {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match (&self.min, &self.max) {
            (0, Max::IntMax(1)) => write!(dest, "?"),
            (0, Max::Unbounded) => write!(dest, "*"),
            (1, Max::Unbounded) => write!(dest, "+"),
            (min, Max::Unbounded) => write!(dest, "{{{},}}", min),
            (min, max) => write!(dest, "{{{}, {}}}", min, max),
        }
    }
}
