use std::cmp;

#[derive(PartialEq, Eq, Clone)]
pub enum Max {
    Unbounded,
    IntMax(usize),
}

impl Max {
    pub fn minus(&self, n: usize) -> Max {
        match self {
            Max::Unbounded => Max::Unbounded,
            Max::IntMax(0) => Max::IntMax(0),
            Max::IntMax(m) => Max::IntMax(cmp::max(m - n, 0)),
        }
    }

    pub fn greater_or_equal(&self, n: usize) -> bool {
        match self {
            Max::IntMax(max) => *max >= n,
            Max::Unbounded => true,
        }
    }
}
