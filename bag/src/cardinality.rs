use std::cmp;

use crate::Max;

pub type Min = usize;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Cardinality {
    pub min: Min,
    pub max: Max,
}

impl Cardinality {
    pub fn nullable(&self) -> bool {
        self.min == 0
    }

    pub fn contains(&self, n: usize) -> bool {
        n >= self.min && self.max.greater_or_equal(n)
    }

    pub fn minus(&self, n: usize) -> Option<Cardinality> {
        if self.contains(n) {
            let min = if self.min > n {
                self.min - n 
            } else {
                0
            };
            Some(Cardinality {
                min: cmp::max(min, 0),
                max: self.max.minus(n),
            })
        } else {
            None
        }
    }
}
