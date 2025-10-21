use rmcp::{
    ErrorData as McpError,
    handler::server::{router::prompt::PromptRouter, wrapper::Parameters},
    model::GetPromptResult,
    prompt, prompt_router,
};

use crate::rudof_mcp_service::service::RudofMcpService;

// Import the public helper functions from the implementation files
use super::node_prompts_impl::*;

#[prompt_router]
impl RudofMcpService {
    #[prompt(
        name = "explore_rdf_node",
        description = "Interactive guide for exploring RDF node information, relationships, and graph structure"
    )]
    async fn explore_rdf_node_prompt(
        &self,
        Parameters(args): Parameters<ExplorerRdfNodePromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        // Delegates the call to the function in node_prompts_impl.rs
        explore_rdf_node_prompt_impl(Parameters(args)).await
    }
}

// Public wrapper to expose the generated prompt router
pub fn prompt_router_public() -> PromptRouter<RudofMcpService> {
    RudofMcpService::prompt_router()
}
