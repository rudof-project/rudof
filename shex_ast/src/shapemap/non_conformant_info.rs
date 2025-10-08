use std::fmt::Display;

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct NonConformantInfo {
    reason: String,
    app_info: Value,
}

impl NonConformantInfo {
    pub fn new(reason: String, app_info: Value) -> Self {
        NonConformantInfo { reason, app_info }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn app_info(&self) -> &Value {
        &self.app_info
    }
}

impl Display for NonConformantInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
