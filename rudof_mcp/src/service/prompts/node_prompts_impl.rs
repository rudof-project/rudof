use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Arguments for the RDF node exploration prompt.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExplorerRdfNodePromptArgs {
    /// RDF node URI/IRI to analyze (e.g., 'http://example.org/resource/123' or 'ex:Person1')
    pub node: String,
    /// Query mode: 'outgoing' (properties of this node), 'incoming' (nodes referencing this), or 'both' (default)
    pub mode: Option<String>,
    /// Optional list of predicate URIs/IRIs to filter the results (e.g., 'foaf:knows')
    pub predicates: Option<Vec<String>>,
}

pub async fn explore_rdf_node_prompt_impl(
    Parameters(args): Parameters<ExplorerRdfNodePromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let node = args.node;
    let mode = args.mode.unwrap_or_else(|| "both".to_string());
    let predicates = args.predicates.unwrap_or_default();

    let predicates_display = if predicates.is_empty() {
        "none (showing all predicates)".to_string()
    } else {
        format!("`{}`", predicates.join("`, `"))
    };

    // Build mode description
    let mode_description = match mode.as_str() {
        "outgoing" => "outgoing relationships only (what this node points to)",
        "incoming" => "incoming relationships only (what points to this node)",
        _ => "both outgoing and incoming relationships",
    };

    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Explore RDF node `{}` with mode: **{}**, predicates: {}",
                node, mode, predicates_display
            ),
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            format!(
                "# 🔍 Exploring RDF Node: `{}`\n\n\
                I'll analyze this node in the loaded RDF graph to discover its relationships and structure.\n\n\
                ## Query Configuration\n\
                - **Node:** `{}`\n\
                - **Mode:** `{}` ({})\n\
                - **Predicate Filter:** {}\n\n\
                ## What We'll Discover\n\n\
                ### Outgoing Arcs (Properties)\n\
                Properties and values **from** this node → showing what this node \"knows\" or \"has\"\n\
                ```\n\
                {} --predicate--> ?object\n\
                ```\n\n\
                ### Incoming Arcs (References)\n\
                Other nodes that **reference** this node → showing what \"knows about\" this node\n\
                ```\n\
                ?subject --predicate--> {}\n\
                ```\n\n\
                ## Next Steps\n\n\
                **1. Run node_info tool** with these parameters:\n\
                ```json\n\
                {{\n\
                  \"node\": \"{}\",\n\
                  \"mode\": \"{}\",\n\
                  \"predicates\": {}\n\
                }}\n\
                ```\n\n\
                **2. Alternative: Query with SPARQL**\n\
                ```sparql\n\
                # Find all outgoing properties\n\
                SELECT ?predicate ?object WHERE {{\n\
                  <{}> ?predicate ?object .\n\
                }}\n\n\
                # Find all incoming references\n\
                SELECT ?subject ?predicate WHERE {{\n\
                  ?subject ?predicate <{}> .\n\
                }}\n\
                ```\n\n\
                **3. If node not found:**\n\
                - Verify the node exists: `execute_sparql_query` with `ASK {{ <{}> ?p ?o }}`\n\
                - Check prefixes: ensure prefix declarations match your data\n\
                - List all subjects: `SELECT DISTINCT ?s WHERE {{ ?s ?p ?o }} LIMIT 10`\n\n\
                ## Exploration Options\n\n\
                Would you like to:\n\
                - **Proceed with current settings** (mode: `{}`)\n\
                - **Change mode** to `outgoing`, `incoming`, or `both`\n\
                - **Filter by predicates** (e.g., `[\"rdf:type\", \"foaf:knows\"]`)\n\
                - **Explore a different node** in the graph\n\
                - **Visualize** the neighborhood using `export_rdf_plantuml`\n\n\
                Let me know how you'd like to proceed!",
                node,
                node,
                mode,
                mode_description,
                predicates_display,
                node,
                node,
                node,
                mode,
                serde_json::to_string(&predicates).unwrap_or_else(|_| "[]".to_string()),
                node,
                node,
                node,
                mode
            ),
        ),
    ];

    Ok(GetPromptResult {
        description: Some(format!(
            "Explore RDF node {} ({}, predicates: {})",
            node, mode_description, predicates_display
        )),
        messages,
    })
}
