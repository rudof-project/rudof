use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct NodeId {
    n: usize,
}

impl NodeId {
    pub fn new(n: usize) -> NodeId {
        NodeId { n }
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.n)?;
        Ok(())
    }
}
