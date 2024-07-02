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
}
