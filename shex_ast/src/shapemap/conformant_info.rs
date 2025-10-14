use serde::Serialize;
use serde_json::Value;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ConformantInfo {
    reasons: Vec<String>,
    app_info: Vec<Value>,
}

impl ConformantInfo {
    pub fn new(reasons: &[String], app_info: &[Value]) -> Self {
        ConformantInfo {
            reasons: reasons.to_owned(),
            app_info: app_info.to_owned(),
        }
    }

    pub fn reason(&self) -> String {
        self.reasons.join("\n")
    }

    pub fn app_info(&self) -> Value {
        Value::Array(self.app_info.clone())
    }

    pub fn merge(mut self, other: ConformantInfo) -> Self {
        self.reasons.extend(other.reasons);
        self.app_info.extend(other.app_info);
        self
    }
}

impl Display for ConformantInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason())
    }
}
