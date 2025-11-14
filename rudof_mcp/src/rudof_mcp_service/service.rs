use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};

use crate::rudof_mcp_service::{prompts, tools};
use rmcp::{
    RoleServer,
    handler::server::router::{prompt::PromptRouter, tool::ToolRouter},
    model::LoggingLevel,
    service::RequestContext,
};
use rudof_lib::{Rudof, RudofConfig};

/// Configuration for the RudofMcpService
#[derive(Clone, Debug)]
pub struct ServiceConfig {}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {}
    }
}

/// MCP service for Rudof operations
#[derive(Clone)]
pub struct RudofMcpService {
    /// Core Rudof instance for validation and operations
    pub rudof: Arc<Mutex<Rudof>>,
    /// Router for handling tool calls
    pub tool_router: ToolRouter<RudofMcpService>,
    /// Router for handling prompt requests
    pub prompt_router: PromptRouter<RudofMcpService>,
    /// Service configuration
    pub config: Arc<RwLock<ServiceConfig>>,
    /// Track resource subscriptions (URI -> list of subscriber IDs)
    pub resource_subscriptions: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Current minimum log level for MCP logging notifications
    pub current_min_log_level: Arc<RwLock<Option<LoggingLevel>>>,
    /// Current request context (temporarily stored during request handling)
    pub(crate) current_context: Arc<RwLock<Option<RequestContext<RoleServer>>>>,
}

impl RudofMcpService {
    pub fn new() -> Self {
        let rudof_config = RudofConfig::new().unwrap();
        let rudof = Rudof::new(&rudof_config).unwrap();
        Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
            config: Arc::new(RwLock::new(ServiceConfig::default())),
            resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            current_min_log_level: Arc::new(RwLock::new(None)),
            current_context: Arc::new(RwLock::new(None)),
        }
    }

    /// Create a new service with custom configuration
    pub fn with_config(config: ServiceConfig) -> Self {
        let rudof_config = RudofConfig::new().unwrap();
        let rudof = Rudof::new(&rudof_config).unwrap();
        Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
            config: Arc::new(RwLock::new(config)),
            resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            current_min_log_level: Arc::new(RwLock::new(None)),
            current_context: Arc::new(RwLock::new(None)),
        }
    }

    /// Add a resource subscription
    pub async fn subscribe_resource(&self, uri: String, subscriber_id: String) {
        let mut subs = self.resource_subscriptions.write().await;
        subs.entry(uri).or_insert_with(Vec::new).push(subscriber_id);
    }

    /// Remove a resource subscription
    pub async fn unsubscribe_resource(&self, uri: &str, subscriber_id: &str) {
        let mut subs = self.resource_subscriptions.write().await;
        if let Some(subscribers) = subs.get_mut(uri) {
            subscribers.retain(|id| id != subscriber_id);
            if subscribers.is_empty() {
                subs.remove(uri);
            }
        }
    }

    /// Get all subscribers for a resource
    pub async fn get_resource_subscribers(&self, uri: &str) -> Vec<String> {
        let subs = self.resource_subscriptions.read().await;
        subs.get(uri).cloned().unwrap_or_default()
    }

    /// Send a notification about resource updates using rmcp's notification system
    pub async fn notify_resource_updated(&self, uri: String) {
        let subscribers = self.get_resource_subscribers(&uri).await;

        if subscribers.is_empty() {
            tracing::debug!(uri = %uri, "No subscribers for resource update");
            return;
        }

        // Use rmcp's notification system via the current RequestContext
        let context_guard = self.current_context.read().await;
        if let Some(context) = context_guard.as_ref() {
            if let Err(e) = context
                .peer
                .notify_resource_updated(rmcp::model::ResourceUpdatedNotificationParam {
                    uri: uri.clone(),
                })
                .await
            {
                tracing::warn!(
                    uri = %uri,
                    error = ?e,
                    "Failed to send resource updated notification"
                );
            } else {
                tracing::info!(
                    uri = %uri,
                    subscriber_count = subscribers.len(),
                    "Resource updated notification sent via rmcp"
                );
            }
        } else {
            tracing::debug!(
                uri = %uri,
                subscriber_count = subscribers.len(),
                "Resource updated (no active request context)"
            );
        }
    }

    /// Send a notification that the resources list has changed
    pub async fn notify_resource_list_changed(&self) {
        let context_guard = self.current_context.read().await;
        if let Some(context) = context_guard.as_ref() {
            if let Err(e) = context.peer.notify_resource_list_changed().await {
                tracing::warn!(
                    error = ?e,
                    "Failed to send resource list changed notification"
                );
            } else {
                tracing::info!("Resource list changed notification sent via rmcp");
            }
        } else {
            tracing::debug!("Resource list changed (no active request context)");
        }
    }

    /// Send a notification that the tools list has changed
    pub async fn notify_tool_list_changed(&self) {
        let context_guard = self.current_context.read().await;
        if let Some(context) = context_guard.as_ref() {
            if let Err(e) = context.peer.notify_tool_list_changed().await {
                tracing::warn!(
                    error = ?e,
                    "Failed to send tool list changed notification"
                );
            } else {
                tracing::info!("Tool list changed notification sent via rmcp");
            }
        } else {
            tracing::debug!("Tool list changed (no active request context)");
        }
    }

    /// Send a notification that the prompts list has changed
    pub async fn notify_prompt_list_changed(&self) {
        let context_guard = self.current_context.read().await;
        if let Some(context) = context_guard.as_ref() {
            if let Err(e) = context.peer.notify_prompt_list_changed().await {
                tracing::warn!(
                    error = ?e,
                    "Failed to send prompt list changed notification"
                );
            } else {
                tracing::info!("Prompt list changed notification sent via rmcp");
            }
        } else {
            tracing::debug!("Prompt list changed (no active request context)");
        }
    }

    /// Get completion suggestions for prompt arguments
    pub(crate) fn get_prompt_argument_completions(
        &self,
        _prompt_name: &str,
        _argument_name: &str,
    ) -> Vec<String> {
        // Not implemented yet;For now, return empty
        vec![]
    }

    /// Get completion suggestions for resource URI templates
    pub(crate) fn get_resource_uri_completions(
        &self,
        _uri: &str,
        _argument_name: &str,
    ) -> Vec<String> {
        // Not implemented yet; For now, return empty
        vec![]
    }
}

impl Default for RudofMcpService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task::spawn_blocking;

    // Initialize the RudofMcpService in a blocking-safe context
    async fn create_test_service() -> RudofMcpService {
        spawn_blocking(|| {
            let rudof_config = rudof_lib::RudofConfig::new().unwrap();
            let rudof = rudof_lib::Rudof::new(&rudof_config).unwrap();
            RudofMcpService {
                rudof: Arc::new(Mutex::new(rudof)),
                tool_router: tools::tool_router_public(),
                prompt_router: prompts::prompt_router_public(),
                config: Arc::new(RwLock::new(ServiceConfig::default())),
                resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
                current_min_log_level: Arc::new(RwLock::new(None)),
                current_context: Arc::new(RwLock::new(None)),
            }
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_rudof_mcp_service_new() {
        let service = create_test_service().await;

        let _rudof_guard = service.rudof.lock().await;

        assert!(
            !service.tool_router.map.is_empty(),
            "ToolRouter map should have routes"
        );
        assert!(
            !service.prompt_router.map.is_empty(),
            "PromptRouter map should have routes"
        );
    }

    #[tokio::test]
    async fn test_rudof_mcp_service_clone() {
        let service = create_test_service().await;

        let cloned_service = service.clone();

        let original_ptr = Arc::as_ptr(&service.rudof);
        let cloned_ptr = Arc::as_ptr(&cloned_service.rudof);
        assert_eq!(
            original_ptr, cloned_ptr,
            "Cloned service should share the same Rudof instance"
        );
    }

    #[tokio::test]
    async fn test_default_trait() {
        let default_service = tokio::task::spawn_blocking(RudofMcpService::default)
            .await
            .unwrap();

        let new_service = create_test_service().await;

        let default_ptr = Arc::as_ptr(&default_service.rudof);
        let new_ptr = Arc::as_ptr(&new_service.rudof);
        assert_ne!(
            default_ptr, new_ptr,
            "Each service should have its own Rudof instance"
        );
    }
}
