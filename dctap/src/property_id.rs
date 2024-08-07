use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct PropertyId {
    str: String,
    line: u64,
}

impl PropertyId {
    pub fn new(str: &str, line: u64) -> PropertyId {
        PropertyId {
            str: str.to_string(),
            line,
        }
    }

    // Represent a property id as a local name in a URI
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

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn str(&self) -> &str {
        self.str.as_str()
    }
}

impl Display for PropertyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)?;
        Ok(())
    }
}
