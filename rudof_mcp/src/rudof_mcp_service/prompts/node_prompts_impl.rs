use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    let node_example = args.node;
    let mode_value = args.mode.unwrap_or_else(|| "both".to_string());
    let predicates_list = args.predicates.unwrap_or_else(|| vec![]);
    let predicates_display = if predicates_list.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", predicates_list.join(", "))
    };

    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Explore RDF Node:\n\
                - **Node:** {}\n\
                - **Mode:** {}\n\
                - **Predicates:** {}",
                node_example, mode_value, predicates_display
            )
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            format!(
                "I'll help you explore the RDF node '{}' and its relationships in the **loaded RDF graph (resource: rdf://graph)**.\n\n\
                Let me retrieve the node information using the **node_info** tool:\n\n\
                **What we'll discover:**\n\
                - **Outgoing arcs**: Properties and values associated with this node (what this node \"says\" about itself)\n\
                - **Incoming arcs**: Other nodes that reference this node (what others \"say\" about this node)\n\n\
                This will give you a complete picture of how '{}' fits into the RDF graph structure.\n\n\
                You can run the **node_info** tool with the following parameters:\n\
                - node: \"{}\"\n\
                - mode: \"{}\" (outgoing | incoming | both [default])\n\
                - predicates: {} (optional - filter by specific predicates)\n\n\
                What would you like to do next?
                1. View all relationships (current mode: **{}**)
                2. Filter by specific predicates (current: **{}**)
                3. Explore a different node
                4. Change the mode to focus only on outgoing or incoming arcs",
                node_example,
                node_example,
                node_example,
                mode_value,
                predicates_display,
                mode_value,
                predicates_display,
            )
        ),
    ];

    Ok(GetPromptResult {
        description: Some(format!(
            "Information for node {} with query mode {} and predicates {}",
            node_example, mode_value, predicates_display
        )),
        messages,
    })
}
