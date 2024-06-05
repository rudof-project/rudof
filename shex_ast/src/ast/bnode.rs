use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
pub struct BNode {
    value: String,
}

impl BNode {
    pub fn new(s: &str) -> BNode {
        BNode {
            value: s.to_string(),
        }
    }
}

impl TryFrom<&str> for BNode {
    type Error = Void;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(BNode {
            value: s.to_string(),
        })
    }
}

impl Into<String> for BNode {
    fn into(self) -> String {
        format!("_:{}", self.value)
    }
}

impl Display for BNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "_:{}", self.value)
    }
}
