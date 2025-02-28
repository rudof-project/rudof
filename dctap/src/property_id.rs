use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct PropertyId {
    str: String,
    line: u64,
}

impl PropertyId {
    pub fn new(str: &str, line: u64) -> PropertyId {
        PropertyId {
            str: replace_newline_by_space(str),
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

impl Display for PropertyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)?;
        Ok(())
    }
}

fn replace_newline_by_space(str: &str) -> String {
    str::replace(str, "\n", " ")
}
