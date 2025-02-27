use serde::{Deserialize, Serialize};

/// Different types of Placeholder resolvers
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum PlaceholderResolver {
    /// Stem resolver simply adds the generated id to the stem
    Stem { stem: String },
}

impl PlaceholderResolver {
    pub fn stem(stem: &str) -> PlaceholderResolver {
        PlaceholderResolver::Stem {
            stem: stem.to_string(),
        }
    }

    pub fn generate(&self, id: u64) -> String {
        match self {
            PlaceholderResolver::Stem { stem } => format!("{stem}{id}"),
        }
    }
}

impl Default for PlaceholderResolver {
    fn default() -> Self {
        PlaceholderResolver::Stem {
            stem: "".to_string(),
        }
    }
}
