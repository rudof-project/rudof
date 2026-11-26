use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ExtendsId {
    str: String,
    label: Option<String>,
    line: u64,
}

impl ExtendsId {
    pub fn new(str: &str, line: u64) -> ExtendsId {
        ExtendsId {
            str: str.to_string(),
            label: None,
            line,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.str.is_empty()
    }

    pub fn str(&self) -> &str {
        self.str.as_str()
    }

    pub fn add_label(&mut self, label: &str) {
        self.label = Some(label.to_string())
    }

    pub fn line(&self) -> u64 {
        self.line
    }
}

impl Display for ExtendsId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.label {
            write!(f, "{} ({})", label, self.str)
        } else {
            write!(f, "{}", self.str)
        }
    }
}
