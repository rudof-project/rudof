use serde::Serialize;
use serde_json::Value;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ConformantInfo {
    reason: String,
    app_info: Value,
}

impl ConformantInfo {
    pub fn new(reason: String, app_info: Value) -> Self {
        ConformantInfo { reason, app_info }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn app_info(&self) -> &Value {
        &self.app_info
    }

    pub fn merge(&self, other: ConformantInfo) -> ConformantInfo {
        let merged_reason = format!("{}\n{}", self.reason, other.reason);
        ConformantInfo {
            reason: merged_reason,
            app_info: match &self.app_info {
                Value::Array(values) => {
                    let mut new_values = values.clone();
                    if let Value::Array(other_values) = other.app_info {
                        new_values.extend(other_values);
                    } else {
                        new_values.push(other.app_info);
                    }
                    Value::Array(new_values)
                }
                _ => todo!(),
            },
        }
    }
}

impl Display for ConformantInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
