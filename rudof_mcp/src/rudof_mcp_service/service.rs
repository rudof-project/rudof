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
        let rudof = Rudof::new(&RudofConfig::new()).unwrap();
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
