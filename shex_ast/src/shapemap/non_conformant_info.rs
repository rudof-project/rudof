use std::fmt::Display;

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct NonConformantInfo {
    errors: Vec<String>,
    app_info: Vec<Value>,
}

impl NonConformantInfo {
    pub fn new(errors: &[String], app_info: &[Value]) -> Self {
        NonConformantInfo {
            errors: errors.to_vec(),
            app_info: app_info.to_vec(),
        }
    }

    pub fn reason(&self) -> String {
        self.errors
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join("\n")
    }

    pub fn app_info(&self) -> Value {
        Value::Array(self.app_info.clone())
    }

    pub fn merge(mut self, other: NonConformantInfo) -> Self {
        self.errors.extend(other.errors);
        self.app_info.extend(other.app_info);
        self
    }
}

impl Display for NonConformantInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason())
    }
}
