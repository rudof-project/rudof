use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct BNode {
    value: String,
}

impl BNode {
    pub fn new(s: &str) -> BNode {
        BNode { value: s.to_string() }
    }
}

impl From<&str> for BNode {
    fn from(s: &str) -> Self {
        BNode { value: s.to_string() }
    }
}

impl Display for BNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "_:{}", self.value)
    }
}
