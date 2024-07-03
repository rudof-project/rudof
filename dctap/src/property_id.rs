use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct PropertyId {
    str: String,
}

impl PropertyId {
    pub fn new(str: &str) -> PropertyId {
        PropertyId {
            str: str.to_string(),
        }
    }

    // Represent a property id as a local name in a URI
    pub fn as_local_name(&self) -> String {
        // TODO: Check how to escape special characters
        self.str.to_string()
    }
}
