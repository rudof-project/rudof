use rmcp::model::LoggingLevel;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Map of MCP log levels to numeric values
/// Based on RFC 5424 syslog severity levels
const LOG_LEVEL_MAP: &[(LoggingLevel, u8)] = &[
    (LoggingLevel::Emergency, 0),
    (LoggingLevel::Alert, 1),
    (LoggingLevel::Critical, 2),
    (LoggingLevel::Error, 3),
    (LoggingLevel::Warning, 4),
    (LoggingLevel::Notice, 5),
    (LoggingLevel::Info, 6),
    (LoggingLevel::Debug, 7),
];

/// Convert LoggingLevel to numeric value
fn level_to_value(level: LoggingLevel) -> u8 {
    LOG_LEVEL_MAP
        .iter()
        .find(|(l, _)| *l == level)
        .map(|(_, v)| *v)
        .unwrap_or(6) // Default to Info if not found
}

/// Check if a log message should be sent based on the current minimum level
pub fn should_log(level: LoggingLevel, min_level: LoggingLevel) -> bool {
    let numeric_level = level_to_value(level);
    let min_numeric_level = level_to_value(min_level);
    numeric_level <= min_numeric_level
}

/// Helper struct for structured logging data
#[derive(Debug, Clone)]
pub struct LogData {
    pub message: String,
    pub fields: Vec<(String, Value)>,
}

impl LogData {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.fields.push((key.into(), value.into()));
        self
    }

    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("message".to_string(), json!(self.message));
        for (key, value) in &self.fields {
            map.insert(key.clone(), value.clone());
        }
        json!(map)
    }
}

/// Send a log message through the MCP protocol if the log level allows it
pub async fn send_log(
    level: LoggingLevel,
    logger: Option<String>,
    data: LogData,
    min_level: Arc<RwLock<Option<LoggingLevel>>>,
    peer: &rmcp::service::Peer<rmcp::RoleServer>,
) {
    tracing::debug!(
        "Preparing to send MCP log: level={:?}, logger={:?}, data={:?}",
        level,
        logger,
        data
    );
    let current_min = min_level.read().await;
    if let Some(min) = *current_min {
        if !should_log(level, min) {
            tracing::debug!(
                "Log level {:?} is below minimum {:?}, not sending log",
                level,
                min
            );
            return;
        }
    } else {
        tracing::debug!("No minimum log level set, not sending MCP log");
        return;
    }
    drop(current_min);

    if let Err(e) = peer
        .notify_logging_message(rmcp::model::LoggingMessageNotificationParam {
            level,
            logger,
            data: data.to_json(),
        })
        .await
    {
        tracing::error!(
            error = ?e,
            level = ?level,
            "Failed to send MCP log notification"
        );
    }
}
