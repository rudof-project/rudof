use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct ShapeLabelIdx(usize);

impl ShapeLabelIdx {
    pub fn new(idx: usize) -> Self {
        ShapeLabelIdx(idx)
    }
}

impl Display for ShapeLabelIdx {
    fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(dest, "{}", self.0)
    }
}

impl From<usize> for ShapeLabelIdx {
    fn from(idx: usize) -> Self {
        ShapeLabelIdx(idx)
    }
}
