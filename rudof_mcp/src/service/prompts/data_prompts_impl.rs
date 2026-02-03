use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Arguments for the RDF data analysis prompt.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeRdfDataPromptArgs {
    /// Focus area for analysis: 'structure', 'quality', 'statistics', or 'all' (default).
    pub focus: Option<String>,
}

pub async fn analyze_rdf_data_prompt_impl(
    Parameters(args): Parameters<AnalyzeRdfDataPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let focus = args.focus.unwrap_or_else(|| "all".to_string());
    
    let focus_description = match focus.as_str() {
        "structure" => "graph structure and relationships",
        "quality" => "data quality and consistency",
        "statistics" => "quantitative statistics",
        _ => "comprehensive overview",
    };

    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!("Analyze the loaded RDF data, focusing on: {}", focus_description),
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            format!(
                "# 📊 RDF Data Analysis Guide\n\n\
                I'll help you analyze the RDF data currently loaded in Rudof, focusing on **{}**.\n\n\
                ## Recommended Analysis Steps\n\n\
                ### 1. Overview - Export and Inspect Data\n\
                ```json\n\
                {{ \"tool\": \"export_rdf_data\", \"format\": \"turtle\" }}\n\
                ```\n\n\
                ### 2. Basic Statistics with SPARQL\n\
                ```sparql\n\
                # Count total triples\n\
                SELECT (COUNT(*) AS ?triples) WHERE {{ ?s ?p ?o }}\n\
                ```\n\
                ```sparql\n\
                # Count distinct subjects, predicates, objects\n\
                SELECT \n\
                  (COUNT(DISTINCT ?s) AS ?subjects)\n\
                  (COUNT(DISTINCT ?p) AS ?predicates)\n\
                  (COUNT(DISTINCT ?o) AS ?objects)\n\
                WHERE {{ ?s ?p ?o }}\n\
                ```\n\n\
                ### 3. Discover Types and Classes\n\
                ```sparql\n\
                SELECT ?type (COUNT(?s) AS ?count)\n\
                WHERE {{ ?s a ?type }}\n\
                GROUP BY ?type\n\
                ORDER BY DESC(?count)\n\
                ```\n\n\
                ### 4. Explore Predicates Used\n\
                ```sparql\n\
                SELECT ?predicate (COUNT(*) AS ?usage)\n\
                WHERE {{ ?s ?predicate ?o }}\n\
                GROUP BY ?predicate\n\
                ORDER BY DESC(?usage)\n\
                ```\n\n\
                ### 5. Visualize Graph Structure\n\
                ```json\n\
                {{ \"tool\": \"export_plantuml\" }}\n\
                ```\n\
                or\n\
                ```json\n\
                {{ \"tool\": \"export_image\", \"image_format\": \"svg\" }}\n\
                ```\n\n\
                ### 6. Inspect Specific Nodes\n\
                Use `node_info` with a node IRI to explore relationships:\n\
                ```json\n\
                {{ \"tool\": \"node_info\", \"node\": \"<IRI>\", \"mode\": \"both\" }}\n\
                ```\n\n\
                ## Next Steps\n\n\
                Would you like me to:\n\
                - **Run these queries** to gather statistics\n\
                - **Explore a specific node** in detail\n\
                - **Validate against a schema** (ShEx or SHACL)\n\
                - **Generate a visualization** of the graph",
                focus_description
            ),
        ),
    ];

    Ok(GetPromptResult {
        description: Some(format!("RDF data analysis guide focusing on {}", focus_description)),
        messages,
    })
}
