use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ShapeId {
    str: String,
    line: u64,
}

impl ShapeId {
    pub fn new(str: &str, line: u64) -> ShapeId {
        ShapeId {
            str: str.to_string(),
            line,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.str.is_empty()
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn str(&self) -> &str {
        self.str.as_str()
    }
}

impl Default for ShapeId {
    fn default() -> Self {
        Self {
            str: "default".to_string(),
            line: 0,
        }
    }
}

impl Display for ShapeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}
