use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OptimizeSparqlQueryPromptArgs {
    /// SPARQL query to optimize
    pub query: String,
}

pub async fn optimize_sparql_query_prompt_impl(
    Parameters(args): Parameters<OptimizeSparqlQueryPromptArgs>,
) -> Result<GetPromptResult, McpError> {
    let messages = vec![
        PromptMessage::new_text(
            PromptMessageRole::User,
            format!("Optimize this SPARQL query:\n```sparql\n{}\n```", args.query)
        ),
        PromptMessage::new_text(
            PromptMessageRole::Assistant,
            "I'll help you optimize this SPARQL query for better performance.\n\n\
            **SPARQL Optimization Techniques:**\n\n\
            1. **Order of triple patterns:**\n\
               - Put the most selective patterns first\n\
               - Start with patterns that match fewer triples\n\n\
            2. **Use FILTER efficiently:**\n\
               - Apply FILTERs as early as possible\n\
               - Avoid complex regex when simpler alternatives exist\n\n\
            3. **Limit data early:**\n\
               - Use LIMIT when you don't need all results\n\
               - Consider using EXISTS instead of COUNT for existence checks\n\n\
            4. **Avoid OPTIONAL when possible:**\n\
               - OPTIONAL can be expensive\n\
               - Consider if you really need it\n\n\
            5. **Use property paths wisely:**\n\
               - `ex:prop+` can be expensive for deep hierarchies\n\
               - Consider adding a depth limit\n\n\
            6. **Reduce DISTINCT overhead:**\n\
               - Only use DISTINCT if you actually have duplicates\n\
               - Sometimes restructuring the query avoids duplicates\n\n\
            **To analyze your query:**\n\
            1. Run it with `execute_sparql_query` and check execution time\n\
            2. Test with LIMIT 10 first to see if results are as expected\n\
            3. Try reordering triple patterns\n\
            4. Consider adding indexes if querying large datasets\n\n\
            Would you like me to suggest a specific optimization for your query?"
        ),
    ];

    Ok(GetPromptResult {
        description: Some("SPARQL query optimization suggestions".to_string()),
        messages,
    })
}
