use std::sync::Arc;
use tokio::sync::Mutex;

use crate::rudof_mcp_service::{prompts, tools};
use rmcp::handler::server::router::{prompt::PromptRouter, tool::ToolRouter};
use rudof_lib::{Rudof, RudofConfig};

#[derive(Clone)]
pub struct RudofMcpService {
    pub rudof: Arc<Mutex<Rudof>>,
    pub tool_router: ToolRouter<RudofMcpService>,
    pub prompt_router: PromptRouter<RudofMcpService>,
}

impl RudofMcpService {
    pub fn new() -> Self {
        // TODO: Check and protect against possible initialization errors
        let rudof_config = RudofConfig::new().unwrap();
        let rudof = Rudof::new(&rudof_config).unwrap();
        Self {
            rudof: Arc::new(Mutex::new(rudof)),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
        }
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
            }
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_rudof_mcp_service_new() {
        let service = create_test_service().await;

        let _rudof_guard = service.rudof.lock().await;

        assert!(!service.tool_router.map.is_empty(), "ToolRouter map should have routes");
        assert!(!service.prompt_router.map.is_empty(), "PromptRouter map should have routes");
    }

    #[tokio::test]
    async fn test_rudof_mcp_service_clone() {
        let service = create_test_service().await;

        let cloned_service = service.clone();

        let original_ptr = Arc::as_ptr(&service.rudof);
        let cloned_ptr = Arc::as_ptr(&cloned_service.rudof);
        assert_eq!(original_ptr, cloned_ptr, "Cloned service should share the same Rudof instance");
    }

    #[tokio::test]
    async fn test_default_trait() {
        let default_service = tokio::task::spawn_blocking(|| RudofMcpService::default()).await.unwrap();

        let new_service = create_test_service().await;

        let default_ptr = Arc::as_ptr(&default_service.rudof);
        let new_ptr = Arc::as_ptr(&new_service.rudof);
        assert_ne!(default_ptr, new_ptr, "Each service should have its own Rudof instance");
    }
}
