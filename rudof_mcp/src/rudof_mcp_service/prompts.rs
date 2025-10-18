use rmcp::{
    ErrorData as McpError, RoleServer, handler::server::wrapper::Parameters, model::*, prompt,
    prompt_router, service::RequestContext,
};

use crate::rudof_mcp_service::types::*;

#[prompt_router]
impl super::RudofMcpService {
    /// This is an example prompt that takes one required argument, message
    #[prompt(name = "example_prompt")]
    async fn example_prompt(
        &self,
        Parameters(args): Parameters<ExamplePromptArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        let prompt = format!(
            "This is an example prompt with your message here: '{}'",
            args.message
        );
        Ok(vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(prompt),
        }])
    }
}

/// Public wrapper to expose the generated prompt router
pub fn prompt_router_public()
-> rmcp::handler::server::router::prompt::PromptRouter<super::RudofMcpService> {
    super::RudofMcpService::prompt_router()
}
