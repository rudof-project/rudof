use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tracing_subscriber::reload;

use crate::rudof_mcp_service::{prompts, tools};
use rmcp::handler::server::router::{prompt::PromptRouter, tool::ToolRouter};
use rudof_lib::{Rudof, RudofConfig};

/// Type alias for the reload handle used to dynamically change log levels
pub type ReloadHandle = reload::Handle<
    tracing_subscriber::EnvFilter,
    tracing_subscriber::Registry,
>;

/// Configuration for the RudofMcpService
#[derive(Clone, Debug)]
pub struct ServiceConfig {
    /// Whether to allow dynamic updates to tools/prompts
    pub allow_dynamic_updates: bool,
    /// Maximum number of concurrent validation operations
    pub max_concurrent_validations: usize,
    /// Enable caching for validation results
    pub cache_enabled: bool,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            allow_dynamic_updates: false,
            max_concurrent_validations: 10,
            cache_enabled: true,
        }
    }
}

/// Main MCP service for Rudof operations
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
            log_level_handle: None,
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
            log_level_handle: None,
        }
    }

    /// Create a new service with a log level handle for dynamic log control
    pub fn with_log_handle(log_handle: Arc<RwLock<ReloadHandle>>) -> Self {
        let rudof_config = RudofConfig::new().unwrap();
        let rudof = Rudof::new(&rudof_config).unwrap();
        Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
            config: Arc::new(RwLock::new(ServiceConfig::default())),
            resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
            log_level_handle: Some(log_handle),
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
                log_level_handle: None,
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
