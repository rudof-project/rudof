use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeRdfDataPromptArgs {}

pub async fn analyze_rdf_data_prompt_impl(
    Parameters(_args): Parameters<AnalyzeRdfDataPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            "Analyze the loaded RDF data"
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            "I'll help you analyze the RDF data currently loaded in Rudof.\n\n\
            **Analysis steps:**\n\
            1. **Export the data** using the `export_rdf_data` tool to see the current triples.\n\
            2. **Run SPARQL queries** using `execute_sparql_query` to explore the loaded data.
            3. **Explore specific nodes** using `node_info` to understand relationships.\n\
            4. **Visualize** using `export_rdf_plantuml` or `export_rdf_image` for a graph overview.\n\n\
            Let me know what aspect you'd like to explore first!"
        ),
    ];

    Ok(GetPromptResult {
        description: Some("Comprehensive analysis of loaded RDF data".to_string()),
        messages,
    })
}