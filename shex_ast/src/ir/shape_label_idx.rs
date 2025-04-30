use std::fmt::Display;

use rbe::Ref;
use serde::Serialize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Serialize)]
pub struct ShapeLabelIdx(usize);

impl Default for ShapeLabelIdx {
    // We start indexes by 1, reserving 0 for internal errors
    fn default() -> Self {
        ShapeLabelIdx(1)
    }
}

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
        match self {
            ShapeLabelIdx(0) => write!(dest, "ERROR"),
            ShapeLabelIdx(n) => write!(dest, "{n}"),
        }
    }
}

impl From<usize> for ShapeLabelIdx {
    fn from(idx: usize) -> Self {
        ShapeLabelIdx(idx)
    }
}
