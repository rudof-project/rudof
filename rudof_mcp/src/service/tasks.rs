//! Async task management for long-running MCP operations (SEP-1686).
//!
//! This module implements the MCP task protocol extension for managing
//! long-running operations like:
//!
//! - Large RDF dataset validation
//! - Complex SPARQL queries over remote endpoints
//! - Batch schema conversions
//!
//! # Usage
//!
//! Tasks are created via `tasks/create` and monitored via `tasks/get`.
//! Clients should poll at the suggested `poll_interval` until the task
//! reaches a terminal state (Completed or Failed).
//!
//! # TTL and Cleanup
//!
//! Tasks have a default TTL of 1 hour. After expiration, task results
//! may be garbage collected by the server.

use rmcp::model::{
    CallToolResult, CancelTaskParams, CreateTaskResult, EmptyObject,
    GetTaskInfoParams, GetTaskInfoResult, GetTaskResultParams, ListTasksResult, PaginatedRequestParams,
    Task, TaskResult, TaskStatus,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Default task time-to-live in milliseconds (1 hour).
///
/// After this duration, completed task results may be garbage collected.
const DEFAULT_TASK_TTL_MS: u64 = 3600000;

/// Suggested client poll interval in milliseconds (5 seconds).
///
/// Clients should wait at least this long between status checks
/// to avoid overwhelming the server with requests.
const DEFAULT_POLL_INTERVAL_MS: u64 = 5000;

/// Internal representation of a task with its metadata and result.
///
/// This struct tracks the full lifecycle of a task from creation
/// through completion or failure.
#[derive(Clone, Debug)]
pub struct TaskEntry {
    /// The MCP Task metadata (ID, status, timestamps).
    pub task: Task,

    /// The task result once completed, or error message if failed.
    ///
    /// `None` while the task is still working.
    pub result: Option<Result<CallToolResult, String>>,
}

impl TaskEntry {
    fn new(task_id: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            task: Task {
                task_id,
                status: TaskStatus::Working,
                status_message: Some("Task enqueued".to_string()),
                created_at: now,
                last_updated_at: None,
                ttl: Some(DEFAULT_TASK_TTL_MS),
                poll_interval: Some(DEFAULT_POLL_INTERVAL_MS),
            },
            result: None,
        }
    }

    fn update_status(&mut self, status: TaskStatus, message: Option<String>) {
        self.task.status = status;
        self.task.status_message = message;
        self.task.last_updated_at = Some(chrono::Utc::now().to_rfc3339());
    }
}

/// Task store for managing async operations
#[derive(Clone, Default)]
pub struct TaskStore {
    tasks: Arc<RwLock<HashMap<String, TaskEntry>>>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Enqueue a new task and return its metadata
    pub async fn enqueue(&self) -> CreateTaskResult {
        let task_id = Uuid::new_v4().to_string();
        let entry = TaskEntry::new(task_id.clone());
        let task = entry.task.clone();

        let mut tasks = self.tasks.write().await;
        tasks.insert(task_id, entry);

        tracing::debug!(task_id = %task.task_id, "Task enqueued");
        CreateTaskResult { task }
    }

    /// List all tasks with optional pagination
    pub async fn list(
        &self,
        params: Option<PaginatedRequestParams>,
    ) -> ListTasksResult {
        let tasks = self.tasks.read().await;
        let all_tasks: Vec<Task> = tasks.values().map(|e| e.task.clone()).collect();

        let (task_list, next_cursor) = if let Some(pagination) = params {
            let page_size = 20;
            let cursor = pagination
                .cursor
                .and_then(|c| c.parse::<usize>().ok())
                .unwrap_or(0);

            let start = cursor;
            let end = std::cmp::min(start + page_size, all_tasks.len());

            let page_tasks = all_tasks[start..end].to_vec();
            let cursor_value = if end < all_tasks.len() {
                Some(end.to_string())
            } else {
                None
            };

            (page_tasks, cursor_value)
        } else {
            (all_tasks, None)
        };

        ListTasksResult {
            tasks: task_list,
            next_cursor,
            ..Default::default()
        }
    }

    /// Get task info by ID
    pub async fn get_info(&self, params: GetTaskInfoParams) -> Option<GetTaskInfoResult> {
        let tasks = self.tasks.read().await;
        tasks.get(&params.task_id).map(|entry| GetTaskInfoResult {
            task: Some(entry.task.clone()),
        })
    }

    /// Get task result if completed
    pub async fn get_result(&self, params: GetTaskResultParams) -> Option<TaskResult> {
        let tasks = self.tasks.read().await;
        let entry = tasks.get(&params.task_id)?;

        match entry.task.status {
            TaskStatus::Completed => {
                if let Some(Ok(result)) = &entry.result {
                    // Convert CallToolResult to TaskResult
                    let value = serde_json::to_value(&result.content).unwrap_or_default();
                    Some(TaskResult {
                        content_type: "application/json".to_string(),
                        value,
                        summary: None,
                    })
                } else {
                    None
                }
            }
            TaskStatus::Failed => {
                if let Some(Err(error_msg)) = &entry.result {
                    Some(TaskResult {
                        content_type: "text/plain".to_string(),
                        value: serde_json::json!({ "error": error_msg }),
                        summary: Some(format!("Task failed: {}", error_msg)),
                    })
                } else {
                    None
                }
            }
            _ => None, // Task still in progress
        }
    }

    /// Cancel a task
    pub async fn cancel(&self, params: CancelTaskParams) -> Option<EmptyObject> {
        let mut tasks = self.tasks.write().await;
        if let Some(entry) = tasks.get_mut(&params.task_id) {
            // Only cancel if not already completed or failed
            match entry.task.status {
                TaskStatus::Working | TaskStatus::InputRequired => {
                    entry.update_status(TaskStatus::Cancelled, Some("Cancelled by client".to_string()));
                    tracing::debug!(task_id = %params.task_id, "Task cancelled");
                    Some(EmptyObject {})
                }
                _ => {
                    tracing::debug!(
                        task_id = %params.task_id,
                        status = ?entry.task.status,
                        "Task cannot be cancelled (already terminal state)"
                    );
                    None
                }
            }
        } else {
            None
        }
    }

    /// Update task status (internal use by task executors)
    pub async fn update_status(
        &self,
        task_id: &str,
        status: TaskStatus,
        message: Option<String>,
    ) -> bool {
        let mut tasks = self.tasks.write().await;
        if let Some(entry) = tasks.get_mut(task_id) {
            entry.update_status(status, message);
            true
        } else {
            false
        }
    }

    /// Complete a task with result
    pub async fn complete(&self, task_id: &str, result: CallToolResult) -> bool {
        let mut tasks = self.tasks.write().await;
        if let Some(entry) = tasks.get_mut(task_id) {
            entry.result = Some(Ok(result));
            entry.update_status(TaskStatus::Completed, Some("Task completed successfully".to_string()));
            tracing::debug!(task_id = %task_id, "Task completed");
            true
        } else {
            false
        }
    }

    /// Fail a task with error
    pub async fn fail(&self, task_id: &str, error: String) -> bool {
        let mut tasks = self.tasks.write().await;
        if let Some(entry) = tasks.get_mut(task_id) {
            entry.result = Some(Err(error.clone()));
            entry.update_status(TaskStatus::Failed, Some(error));
            tracing::debug!(task_id = %task_id, "Task failed");
            true
        } else {
            false
        }
    }
}
