use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeId {
    pub id: usize,
}

impl EdgeId {
    pub fn new(id: usize) -> Self {
        EdgeId { id }
    }
}

impl Display for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}
