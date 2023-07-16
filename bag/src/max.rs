#[derive(PartialEq, Eq, Clone)]
pub enum Max {
    Unbounded,
    IntMax(usize)
}

impl Max {
    pub fn minus_one(&self) -> Max {
        match self {
            Max::Unbounded => Max::Unbounded,
            Max::IntMax(0) => Max::IntMax(0),
            Max::IntMax(m) => Max::IntMax(m - 1)
        }
    }
}
