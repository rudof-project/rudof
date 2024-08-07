use serde_derive::{Deserialize, Serialize};
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

    // Represent a shape id as a local name in a URI
    pub fn as_local_name(&self) -> String {
        // TODO: Check how to escape special characters
        self.str.to_string()
    }

    pub fn as_prefix_local_name(&self) -> Option<(String, String)> {
        // TODO: Check how to escape special characters
        if let Some((prefix, localname)) = self.str.rsplit_once(':') {
            Some((prefix.to_string(), localname.to_string()))
        } else {
            None
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
