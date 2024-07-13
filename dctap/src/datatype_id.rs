use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct DatatypeId {
    str: String,
}

impl DatatypeId {
    pub fn new(str: &str) -> DatatypeId {
        DatatypeId {
            str: str.to_string(),
        }
    }

    // Represent a datatype id as a local name in a URI
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
}

impl Display for DatatypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((prefix, name)) = self.as_prefix_local_name() {
            write!(f, "{prefix}:{name}")?;
        } else {
            write!(f, "{}", self.str)?;
        }
        Ok(())
    }
}
