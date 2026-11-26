use crate::Max;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
// use tracing::trace;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Interval {
    pub n: Max,
    pub m: Max,
}

impl Interval {
    /// Creates a new interval with the given lower bound n and upper bound m.
    pub fn new(n: Max, m: Max) -> Self {
        Interval { n, m }
    }

    pub fn fail() -> Self {
        Interval {
            n: Max::IntMax(1),
            m: Max::IntMax(0),
        }
    }

    pub fn zero_any() -> Self {
        Interval {
            n: Max::IntMax(0),
            m: Max::Unbounded,
        }
    }

    pub fn zero_zero() -> Self {
        Interval {
            n: Max::IntMax(0),
            m: Max::IntMax(0),
        }
    }

    pub fn one_any() -> Self {
        Interval {
            n: Max::IntMax(1),
            m: Max::Unbounded,
        }
    }

    /// Returns true if the interval is empty. An interval [n, m] is empty if n is greater than m,
    /// or if n is Unbounded (no finite value can satisfy t >= Unbounded).
    pub fn is_empty(&self) -> bool {
        matches!(self.n, Max::Unbounded) || self.n.greater_than(&self.m)
    }

    /// Returns true if the interval contains the value v: n <= v <= m.
    pub fn contains(&self, v: usize) -> bool {
        let lower_ok = match &self.n {
            Max::Unbounded => false,
            Max::IntMax(n) => v >= *n,
        };
        lower_ok && self.m.greater_or_equal(v)
    }

    /// Returns the intersection of two intervals. The intersection of two intervals [n1, m1] and [n2, m2] is defined as the largest interval that is contained in both intervals. This can be calculated by taking the maximum of the lower bounds (n1 and n2) and the minimum of the upper bounds (m1 and m2).
    pub fn intersection(&self, other: &Interval) -> Interval {
        let n = self.n.max(&other.n);
        let m = self.m.min(&other.m);
        Interval { n, m }
    }

    /// Returns the union of two intervals. The union of two intervals [n1, m1] and [n2, m2] is defined as the smallest interval that contains both intervals. This can be calculated by taking the minimum of the lower bounds (n1 and n2) and the maximum of the upper bounds (m1 and m2).
    pub fn addition(&self, other: &Interval) -> Interval {
        let n = self.n.plus(&other.n);
        let m = self.m.plus(&other.m);
        // trace!("### Adding intervals: {} + {} = [{}, {}]", self, other, n, m);
        Interval { n, m }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.n, self.m)
    }
}
