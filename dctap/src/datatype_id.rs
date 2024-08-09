use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct DatatypeId {
    str: String,
    line: u64,
}

impl DatatypeId {
    pub fn new(str: &str, line: u64) -> DatatypeId {
        DatatypeId {
            str: str.to_string(),
            line,
        }
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn str(&self) -> &str {
        self.str.as_str()
    }
}

impl Display for DatatypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)?;
        Ok(())
    }
}
