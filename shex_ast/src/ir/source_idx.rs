use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SourceIdx(usize);

impl SourceIdx {
    pub fn new(idx: usize) -> Self {
        SourceIdx(idx)
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl Display for SourceIdx {
    fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(dest, "{}", self.0)
    }
}
