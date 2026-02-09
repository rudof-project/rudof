//! MCP logging notifications with RFC 5424 severity levels.
//!
//! This module implements the MCP logging protocol extension, allowing
//! the server to send structured log messages to connected clients.
//!
//! # Log Levels
//!
//! Log levels follow RFC 5424 (syslog) severity levels, from most to
//! least severe:
//!
//! | Level     | Value | Description                              |
//! |-----------|-------|------------------------------------------|
//! | Emergency | 0     | System is unusable                       |
//! | Alert     | 1     | Action must be taken immediately         |
//! | Critical  | 2     | Critical conditions                      |
//! | Error     | 3     | Error conditions                         |
//! | Warning   | 4     | Warning conditions                       |
//! | Notice    | 5     | Normal but significant condition         |
//! | Info      | 6     | Informational messages                   |
//! | Debug     | 7     | Debug-level messages                     |
//!
//! # Filtering
//!
//! Clients can set a minimum log level via `logging/setLevel`. Only
//! messages at or above the minimum severity are sent. For example,
//! setting the level to "Warning" will send Warning, Error, Critical,
//! Alert, and Emergency messages, but suppress Notice, Info, and Debug.

use rmcp::model::LoggingLevel;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Mapping of MCP log levels to RFC 5424 numeric severity values.
///
/// Lower values indicate higher severity.
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

/// Convert a [`LoggingLevel`] to its numeric RFC 5424 value.
fn level_to_value(level: LoggingLevel) -> u8 {
    LOG_LEVEL_MAP
        .iter()
        .find(|(l, _)| *l == level)
        .map(|(_, v)| *v)
        .unwrap_or(6) // Default to Info if not found
}

/// Check if a log message should be sent based on the current minimum level.
///
/// Returns `true` if the message level is at or above the minimum severity.
///
/// # Arguments
///
/// * `level` - The severity level of the message to send
/// * `min_level` - The minimum level configured by the client
pub fn should_log(level: LoggingLevel, min_level: LoggingLevel) -> bool {
    let numeric_level = level_to_value(level);
    let min_numeric_level = level_to_value(min_level);
    numeric_level <= min_numeric_level
}

/// Builder for structured log message data.
///
/// Supports fluent construction of log messages with custom fields.
#[derive(Debug, Clone)]
pub struct LogData {
    /// The main log message text.
    pub message: String,

    /// Additional structured fields as key-value pairs.
    pub fields: Vec<(String, Value)>,
}

impl LogData {
    /// Create a new log data builder with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            fields: Vec::new(),
        }
    }

    /// Add a custom field to the log data.
    ///
    /// Fields are included in the JSON payload sent to clients.
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.fields.push((key.into(), value.into()));
        self
    }

    /// Convert the log data to a JSON value for the MCP notification.
    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("message".to_string(), json!(self.message));
        for (key, value) in &self.fields {
            map.insert(key.clone(), value.clone());
        }
        json!(map)
    }
}

/// Send a log message to the MCP client if the log level permits.
///
/// This function checks the current minimum log level and only sends
/// the notification if the message severity meets the threshold.
///
/// # Arguments
///
/// * `level` - Severity level of this log message
/// * `logger` - Optional logger name (e.g., "tools", "validation")
/// * `data` - Structured log data with message and fields
/// * `min_level` - Shared state holding the current minimum level
/// * `peer` - The rmcp peer connection to send notifications through
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
            tracing::debug!("Log level {:?} is below minimum {:?}, not sending log", level, min);
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
