use std::sync::Arc;
use tokio::sync::Mutex;

use rmcp::handler::server::router::{tool::ToolRouter, prompt::PromptRouter};
use crate::rudof_mcp_service::{tools, prompts};
use rudof_lib::{Rudof, RudofConfig};

#[derive(Clone)]
pub struct RudofMcpService {
    pub rudof: Arc<Mutex<Rudof>>,
    pub tool_router: ToolRouter<RudofMcpService>,
    pub prompt_router: PromptRouter<RudofMcpService>,
}

impl RudofMcpService {
    pub fn new() -> Self {
        Self {
            rudof: Arc::new(Mutex::new(Rudof::new(&RudofConfig::new()))),
            tool_router: tools::tool_router_public(),
            prompt_router: prompts::prompt_router_public(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_prompt_attributes_generated() {
        let example_attr = RudofMcpService::example_prompt_prompt_attr();
        assert_eq!(example_attr.name, "example_prompt");
        assert!(example_attr.description.is_some());
        assert!(example_attr.arguments.is_some());
        let args = example_attr.arguments.unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].name, "message");
        assert_eq!(args[0].required, Some(true));
    }

    #[tokio::test]
    async fn test_prompt_router_has_routes() {
    let router = crate::rudof_mcp_service::prompts::prompt_router_public();
    assert!(router.has_route("example_prompt"));
    let prompts = router.list_all();
    assert!(prompts.len() >= 1);
    }
}
