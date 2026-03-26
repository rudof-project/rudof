use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub(crate) struct ShapeLabelIdx(usize);

impl ShapeLabelIdx {
    pub fn new(idx: usize) -> Self {
        ShapeLabelIdx(idx)
    }
}

impl Display for ShapeLabelIdx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for ShapeLabelIdx {
    fn from(value: usize) -> Self {
        ShapeLabelIdx(value)
    }
}
