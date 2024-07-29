use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TapConfig {
    delimiter: u8,
    quote: u8,
    flexible: bool,
    picklist_delimiter: String,
}

impl TapConfig {
    pub fn picklist_delimiter(&self) -> &str {
        &self.picklist_delimiter
    }

    pub fn delimiter(&self) -> u8 {
        self.delimiter
    }

    pub fn quote(&self) -> u8 {
        self.quote
    }

    pub fn flexible(&self) -> bool {
        self.flexible
    }
}

impl Default for TapConfig {
    fn default() -> Self {
        Self {
            picklist_delimiter: "|".to_string(),
            delimiter: b',',
            flexible: false,
            quote: b'"',
        }
    }
}
