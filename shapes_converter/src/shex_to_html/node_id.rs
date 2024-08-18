use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash, Serialize, Deserialize)]
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
