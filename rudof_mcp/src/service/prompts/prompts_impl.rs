use rmcp::{
    ErrorData as McpError,
    handler::server::{router::prompt::PromptRouter, wrapper::Parameters},
    model::GetPromptResult,
    prompt, prompt_router,
};

use crate::service::mcp_service::RudofMcpService;

// Import the public helper functions from the implementation files
use crate::service::prompts::data_prompts_impl::*;
use crate::service::prompts::node_prompts_impl::*;
use crate::service::prompts::validation_prompts_impl::*;

#[prompt_router]
impl RudofMcpService {
    /// Interactive guide for exploring RDF node information
    #[prompt(
        name = "explore_rdf_node",
        description = "Interactive guide for exploring RDF node information, relationships, and graph structure"
    )]
    async fn explore_rdf_node_prompt(
        &self,
        Parameters(args): Parameters<ExplorerRdfNodePromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        explore_rdf_node_prompt_impl(Parameters(args)).await
    }

    /// Comprehensive guide for analyzing loaded RDF data
    #[prompt(
        name = "analyze_rdf_data",
        description = "Comprehensive guide for analyzing loaded RDF data structure, patterns, and quality"
    )]
    async fn analyze_rdf_data_prompt(
        &self,
        Parameters(args): Parameters<AnalyzeRdfDataPromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        analyze_rdf_data_prompt_impl(Parameters(args)).await
    }

    /// Guide for validating RDF data against ShEx or SHACL schemas
    #[prompt(
        name = "validation_guide",
        description = "Step-by-step guide for validating RDF data against ShEx or SHACL schemas"
    )]
    async fn validation_guide_prompt(
        &self,
        Parameters(args): Parameters<ValidationGuidePromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        validation_guide_prompt_impl(Parameters(args)).await
    }

    /// Helper for building SPARQL queries
    #[prompt(
        name = "sparql_builder",
        description = "Interactive helper for building and understanding SPARQL queries"
    )]
    async fn sparql_builder_prompt(
        &self,
        Parameters(args): Parameters<SparqlBuilderPromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        sparql_builder_prompt_impl(Parameters(args)).await
    }
}

/// Public wrapper to expose the generated prompt router
pub fn prompt_router_public() -> PromptRouter<RudofMcpService> {
    RudofMcpService::prompt_router()
}
