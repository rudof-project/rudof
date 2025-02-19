use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::Max;
use crate::Min;
use std::cmp;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Cardinality {
    pub min: Min,
    pub max: Max,
}

impl Cardinality {
    pub fn from(min: Min, max: Max) -> Cardinality {
        Cardinality { min, max }
    }

    pub fn nullable(&self) -> bool {
        self.min == Min { value: 0 }
    }

    pub fn is_0_0(&self) -> bool {
        self.min == Min { value: 0 } && self.max == Max::IntMax(0)
    }

    pub fn is_1_1(&self) -> bool {
        self.min == Min { value: 1 } && self.max == Max::IntMax(1)
    }

    pub fn is_star(&self) -> bool {
        self.min == Min { value: 0 } && self.max == Max::Unbounded
    }

    pub fn is_plus(&self) -> bool {
        self.min == Min { value: 1 } && self.max == Max::Unbounded
    }

    pub fn contains(&self, n: usize) -> bool {
        n >= self.min.value && self.max.greater_or_equal(n)
    }

    pub fn minus(&self, n: usize) -> Cardinality {
        let min = if self.min.value > n {
            self.min.value - n
        } else {
            0
        };
        Cardinality {
            min: Min {
                value: cmp::max(min, 0),
            },
            max: self.max.minus(n),
        }
    }
}

impl fmt::Display for Cardinality {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match (&self.min, &self.max) {
            (Min { value: 0 }, Max::IntMax(1)) => write!(dest, "?"),
            (Min { value: 0 }, Max::Unbounded) => write!(dest, "*"),
            (Min { value: 1 }, Max::Unbounded) => write!(dest, "+"),
            (min, Max::Unbounded) => write!(dest, "{{{},}}", min.value),
            (min, max) => write!(dest, "{{{}, {}}}", min.value, max),
        }
    }
}

impl fmt::Debug for Cardinality {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match (&self.min, &self.max) {
            (Min { value: 0 }, Max::IntMax(1)) => write!(dest, "?"),
            (Min { value: 0 }, Max::Unbounded) => write!(dest, "*"),
            (Min { value: 1 }, Max::Unbounded) => write!(dest, "+"),
            (min, Max::Unbounded) => write!(dest, "{{{},}}", min.value),
            (min, max) => write!(dest, "{{{}, {}}}", min.value, max),
        }
    }
}
