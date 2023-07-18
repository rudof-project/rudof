use std::{cmp, fmt};

#[derive(PartialEq, Eq, Clone, Debug)]
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

impl From<usize> for Max {
    fn from(m: usize) -> Self {
        Max::IntMax(m)
    }
}

impl fmt::Display for Max {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Max::Unbounded => write!(dest, "*"),
            Max::IntMax(max) => write!(dest, "{max}"),
        }
    }
}
