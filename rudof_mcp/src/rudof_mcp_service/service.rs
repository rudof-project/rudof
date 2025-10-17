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

