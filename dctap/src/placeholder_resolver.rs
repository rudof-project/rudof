use serde::{Deserialize, Serialize};

/// Different types of Placeholder resolvers
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderResolver {
    /// Stem resolver simply adds the generated id to the stem
    Stem(String),
}

impl PlaceholderResolver {
    pub fn new() -> Self {
        Self::stem("")
    }

    pub fn stem(stem: &str) -> PlaceholderResolver {
        PlaceholderResolver::Stem(stem.to_string())
    }

    pub fn generate(&self, id: u64) -> String {
        match self {
            PlaceholderResolver::Stem(stem) => format!("{stem}{id}"),
        }
    }
}

impl Default for PlaceholderResolver {
    fn default() -> Self {
        Self::new()
    }
}
