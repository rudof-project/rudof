use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TapConfig {
    picklist_delimiter: String,
}

impl TapConfig {
    pub fn picklist_delimiter(&self) -> &str {
        &self.picklist_delimiter
    }
}

impl Default for TapConfig {
    fn default() -> Self {
        Self {
            picklist_delimiter: "|".to_string(),
        }
    }
}
