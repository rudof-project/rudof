use rmcp::{
    ErrorData as McpError,
    handler::server::{router::prompt::PromptRouter, wrapper::Parameters},
    model::GetPromptResult,
    prompt, prompt_router,
};

use crate::rudof_mcp_service::service::RudofMcpService;

// Import the public helper functions from the implementation files
use super::data_prompts_impl::*;
use super::node_prompts_impl::*;
use super::query_prompts_impl::*;
use super::validation_prompts_impl::*;

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

    #[prompt(
        name = "generate_test_data",
        description = "Generate conformant RDF test data examples from a ShEx schema"
    )]
    async fn generate_test_data_prompt(
        &self,
        Parameters(args): Parameters<GenerateTestDataPromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        generate_test_data_prompt_impl(Parameters(args)).await
    }

    #[prompt(
        name = "optimize_sparql_query",
        description = "Get suggestions to optimize SPARQL query performance and efficiency"
    )]
    async fn optimize_sparql_query_prompt(
        &self,
        Parameters(args): Parameters<OptimizeSparqlQueryPromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        optimize_sparql_query_prompt_impl(Parameters(args)).await
    }

    #[prompt(
        name = "suggest_shex_schema",
        description = "Get help creating a ShEx schema for your RDF data domain"
    )]
    async fn suggest_shex_schema_prompt(
        &self,
        Parameters(args): Parameters<SuggestShexSchemaPromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        suggest_shex_schema_prompt_impl(Parameters(args)).await
    }

    #[prompt(
        name = "explain_validation_errors",
        description = "Understand and fix ShEx validation errors with detailed explanations"
    )]
    async fn explain_validation_errors_prompt(
        &self,
        Parameters(args): Parameters<ExplainValidationErrorsPromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        explain_validation_errors_prompt_impl(Parameters(args)).await
    }

    #[prompt(
        name = "debug_shex_schema",
        description = "Debug ShEx schema syntax, reference, and logical errors"
    )]
    async fn debug_shex_schema_prompt(
        &self,
        Parameters(args): Parameters<DebugShexSchemaPromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        debug_shex_schema_prompt_impl(Parameters(args)).await
    }
}

// Public wrapper to expose the generated prompt router
pub fn prompt_router_public() -> PromptRouter<RudofMcpService> {
    RudofMcpService::prompt_router()
}
