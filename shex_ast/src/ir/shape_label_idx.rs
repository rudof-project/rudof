use rbe::Ref;
use serde::Serialize;
use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default, Serialize)]
pub struct ShapeLabelIdx(usize);

impl Ref for ShapeLabelIdx {}

impl ShapeLabelIdx {
    pub fn incr(&mut self) {
        self.0 += 1;
    }

    pub fn error() -> ShapeLabelIdx {
        ShapeLabelIdx(0)
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
