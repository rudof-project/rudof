use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ShapeId {
    str: String,
}

impl ShapeId {
    pub fn new(str: &str) -> ShapeId {
        ShapeId {
            str: str.to_string(),
        }
    }

    // Represent a shape id as a local name in a URI
    pub fn as_local_name(&self) -> String {
        // TODO: Check how to escape special characters
        self.str.to_string()
    }

    pub fn is_empty(&self) -> bool {
        self.str.is_empty()
    }
}

impl Default for ShapeId {
    fn default() -> Self {
        Self {
            str: "default".to_string(),
        }
    }
}

impl Display for ShapeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}
