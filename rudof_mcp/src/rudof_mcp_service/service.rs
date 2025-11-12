use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock, broadcast};
use tracing_subscriber::reload;

use crate::rudof_mcp_service::{prompts, tools};
use rmcp::handler::server::router::{prompt::PromptRouter, tool::ToolRouter};
use rudof_lib::{Rudof, RudofConfig};

/// Type alias for the reload handle used to dynamically change log levels
pub type ReloadHandle = reload::Handle<
    tracing_subscriber::EnvFilter,
    tracing_subscriber::Registry,
>;

/// Notification types that can be sent to clients
#[derive(Debug, Clone)]
pub enum ServerNotification {
    ToolsListChanged,
    PromptsListChanged,
    ResourcesListChanged,
    ResourceUpdated(String), // Resource URI
}

/// Configuration for the RudofMcpService
#[derive(Clone, Debug)]
pub struct ServiceConfig {
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
        }
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
    /// Handle for dynamically reloading log level
    pub log_level_handle: Option<Arc<RwLock<ReloadHandle>>>,
    /// Broadcast channel for sending notifications to clients
    pub notification_tx: Arc<broadcast::Sender<ServerNotification>>,
}

impl RudofMcpService {
    pub fn new() -> Self {
        let rudof_config = RudofConfig::new().unwrap();
        let rudof = Rudof::new(&rudof_config).unwrap();
        let (notification_tx, _) = broadcast::channel(100);
        Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
            config: Arc::new(RwLock::new(ServiceConfig::default())),
            resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            log_level_handle: None,
            notification_tx: Arc::new(notification_tx),
        }
    }

    /// Create a new service with custom configuration
    pub fn with_config(config: ServiceConfig) -> Self {
        let rudof_config = RudofConfig::new().unwrap();
        let rudof = Rudof::new(&rudof_config).unwrap();
        let (notification_tx, _) = broadcast::channel(100);
        Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
            config: Arc::new(RwLock::new(config)),
            resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            log_level_handle: None,
            notification_tx: Arc::new(notification_tx),
        }
    }

    /// Create a new service with a log level handle for dynamic log control
    pub fn with_log_handle(log_handle: Arc<RwLock<ReloadHandle>>) -> Self {
        let rudof_config = RudofConfig::new().unwrap();
        let rudof = Rudof::new(&rudof_config).unwrap();
        let (notification_tx, _) = broadcast::channel(100);
        Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
            config: Arc::new(RwLock::new(ServiceConfig::default())),
            resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            log_level_handle: Some(log_handle),
            notification_tx: Arc::new(notification_tx),
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

    /// Send a notification to all subscribed clients
    pub fn notify(&self, notification: ServerNotification) {
        // Log the notification attempt
        match &notification {
            ServerNotification::ToolsListChanged => {
                tracing::debug!("Sending tools/list_changed notification");
            }
            ServerNotification::PromptsListChanged => {
                tracing::debug!("Sending prompts/list_changed notification");
            }
            ServerNotification::ResourcesListChanged => {
                tracing::debug!("Sending resources/list_changed notification");
            }
            ServerNotification::ResourceUpdated(uri) => {
                tracing::debug!(%uri, "Sending resources/updated notification");
            }
        }
        
        // Send via broadcast channel - ignore errors if no receivers
        let _ = self.notification_tx.send(notification);
    }

    /// Subscribe to notifications
    pub fn subscribe_notifications(&self) -> broadcast::Receiver<ServerNotification> {
        self.notification_tx.subscribe()
    }

    /// Get completion suggestions for prompt arguments
    pub(crate) fn get_prompt_argument_completions(
        &self,
        prompt_name: &str,
        argument_name: &str,
    ) -> Vec<String> {
        match (prompt_name, argument_name) {
            // Completions for explore_rdf_node prompt
            ("explore_rdf_node", "mode") => vec![
                "outgoing".to_string(),
                "incoming".to_string(),
                "both".to_string(),
            ],
            
            // Completions for analyze_rdf_data analysis types
            ("analyze_rdf_data", "analysis_type") => vec![
                "structure".to_string(),
                "patterns".to_string(),
                "quality".to_string(),
                "statistics".to_string(),
            ],
            
            // Completions for generate_test_data complexity
            ("generate_test_data", "complexity") => vec![
                "simple".to_string(),
                "moderate".to_string(),
                "complex".to_string(),
            ],
            
            _ => vec![],
        }
    }

    /// Get completion suggestions for resource URI templates
    pub(crate) fn get_resource_uri_completions(
        &self,
        _uri: &str,
        _argument_name: &str,
    ) -> Vec<String> {
        // Resource URI completions can be extended based on available resources
        // For now, return empty as resources are dynamically generated
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
            let (notification_tx, _) = broadcast::channel(100);
            RudofMcpService {
                rudof: Arc::new(Mutex::new(rudof)),
                tool_router: tools::tool_router_public(),
                prompt_router: prompts::prompt_router_public(),
                config: Arc::new(RwLock::new(ServiceConfig::default())),
                resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
                log_level_handle: None,
                notification_tx: Arc::new(notification_tx),
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
