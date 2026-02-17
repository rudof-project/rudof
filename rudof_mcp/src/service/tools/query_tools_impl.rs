use crate::service::{errors::*, mcp_service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, CreateMessageRequestParams, Role, SamplingMessage},
};
use rudof_lib::{
    InputSpec,
    query::{detect_query_type, execute_query},
    query_result_format::ResultQueryFormat,
    query_type::QueryType,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::str::FromStr;

use super::helpers::*;

/// Request parameters for executing a SPARQL query.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSparqlQueryRequest {
    /// Direct SPARQL query to execute.
    /// Supports SELECT, CONSTRUCT, ASK queries.
    /// Either this or `query_natural_language` must be provided.
    pub query: Option<String>,

    /// Natural language description of what to query.
    /// Will be converted to SPARQL using LLM sampling.
    /// Either this or `query` must be provided.
    pub query_natural_language: Option<String>,

    /// Output format for query results.
    /// Supported: internal, turtle, ntriples, json-ld, rdf-xml, csv, trig, n3, nquads
    /// Default: internal
    pub result_format: Option<String>,
}

/// Response containing query execution results.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryExecutionResponse {
    /// Type of query that was executed (SELECT, CONSTRUCT, ASK)
    pub query_type: String,
    /// Format used for the results
    pub result_format: String,
    /// Execution status
    pub status: String,
    /// Query results as a string
    pub results: String,
    /// Size of the results in bytes
    pub result_size_bytes: usize,
    /// Number of lines in the result
    pub result_lines: usize,
}

/// Generate a SPARQL query from natural language using rmcp sampling.
///
/// Uses the MCP sampling capability to request LLM assistance in
/// converting natural language to SPARQL.
async fn generate_sparql_from_natural_language(
    service: &RudofMcpService,
    natural_language: &str,
) -> Result<String, McpError> {
    let system_message = r#"You are a SPARQL query expert. Convert natural language questions into valid SPARQL queries.
                                    - Only output the SPARQL query, no explanations or markdown formatting
                                    - Use standard SPARQL syntax (SELECT, CONSTRUCT, ASK, or DESCRIBE)
                                    - Include appropriate prefixes if needed
                                    - Make the query efficient and correct
                                    - If you need to guess prefixes, use common ones like rdf:, rdfs:, xsd:, ex:, etc."#;

    let user_message = format!("Generate a SPARQL query for: {}", natural_language);

    // Get the current request context to access the peer for sampling
    let context_guard = service.current_context.read().await;
    let context = context_guard.as_ref().ok_or_else(|| {
        internal_error(
            "Sampling context error",
            "Request context not found",
            Some(json!({"operation":"generate_sparql_from_natural_language","phase":"get_context"})),
        )
    })?;

    // Create sampling request with messages
    let sampling_request = CreateMessageRequestParams {
        meta: None,
        task: None,
        messages: vec![
            SamplingMessage {
                role: Role::User,
                content: Content::text(system_message),
            },
            SamplingMessage {
                role: Role::User,
                content: Content::text(user_message.clone()),
            },
        ],
        model_preferences: None,
        system_prompt: None,
        include_context: None,
        temperature: Some(0.3),
        max_tokens: 512,
        stop_sequences: None,
        metadata: None,
    };

    // Send sampling request through rmcp
    let response = context.peer.create_message(sampling_request).await.map_err(|e| {
        internal_error(
            "Query generation error",
            e.to_string(),
            Some(json!({"operation":"generate_sparql_from_natural_language","phase":"create_message"})),
        )
    })?;

    // Extract text from the SamplingMessage content in the response
    // The response.message.content is of type Content
    let generated_query = if let Some(text_content) = response.message.content.as_text() {
        text_content.text.clone()
    } else {
        return Err(internal_error(
            "Sampling response error",
            "Expected text response from LLM",
            Some(json!({"operation":"generate_sparql_from_natural_language","phase":"extract_response_text"})),
        ));
    };

    // Clean up any markdown code blocks that might have been generated
    let cleaned_query = if generated_query.starts_with("```") {
        generated_query
            .lines()
            .skip(1)
            .take_while(|line| !line.starts_with("```"))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    } else {
        generated_query.trim().to_string()
    };

    tracing::debug!(
        natural_language = %natural_language,
        generated_query = %cleaned_query,
        "Generated SPARQL query from natural language via rmcp sampling"
    );

    Ok(cleaned_query)
}

/// Execute a SPARQL query against the loaded RDF data.
///
/// Supports SELECT, CONSTRUCT, ASK queries.
/// Can accept either a direct SPARQL query or a natural language
/// description that will be converted to SPARQL using LLM sampling.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - Both `query` and `query_natural_language` are provided
/// - Neither `query` nor `query_natural_language` is provided
/// - The query type cannot be determined
/// - The result format is invalid
///
/// Returns a Protocol Error for internal execution failures.
pub async fn execute_sparql_query_impl(
    service: &RudofMcpService,
    Parameters(ExecuteSparqlQueryRequest {
        query,
        query_natural_language,
        result_format,
    }): Parameters<ExecuteSparqlQueryRequest>,
) -> Result<CallToolResult, McpError> {
    // Determine the SPARQL query to execute - return Tool Execution Error for invalid combinations
    let sparql_query = match (query, query_natural_language) {
        (Some(q), None) => {
            // Direct SPARQL query provided
            q
        },
        (None, Some(nl)) => {
            // Natural language query provided - generate SPARQL using rmcp sampling
            generate_sparql_from_natural_language(service, &nl).await?
        },
        (Some(_), Some(_)) => {
            return Ok(ToolExecutionError::new(
                "Cannot provide both 'query' and 'query_natural_language'. Choose one.",
            )
            .into_call_tool_result());
        },
        (None, None) => {
            return Ok(ToolExecutionError::with_hint(
                "No query provided",
                "Provide either 'query' with a SPARQL query string, or 'query_natural_language' with a description",
            )
            .into_call_tool_result());
        },
    };

    // Detect query type - return Tool Execution Error if unrecognizable
    let query_type_str = match detect_query_type(&sparql_query) {
        Some(qt) => qt,
        None => {
            return Ok(ToolExecutionError::with_hint(
                "Cannot determine query type",
                "Ensure the query starts with SELECT, CONSTRUCT, ASK",
            )
            .into_call_tool_result());
        },
    };

    // Parse query type
    let parsed_query_type = match QueryType::from_str(&query_type_str) {
        Ok(qt) => qt,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid query type: {}", e),
                "Supported query types: SELECT, CONSTRUCT, ASK",
            )
            .into_call_tool_result());
        },
    };

    // Parse result format - return Tool Execution Error for invalid format
    let result_format_str = result_format.as_deref().unwrap_or("Internal");
    let parsed_result_format = match ResultQueryFormat::from_str(result_format_str) {
        Ok(fmt) => fmt,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid result format '{}': {}", result_format_str, e),
                format!("Supported formats: {}", SPARQL_RESULT_FORMATS),
            )
            .into_call_tool_result());
        },
    };

    let query_spec = InputSpec::Str(sparql_query.clone());

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    execute_query(
        &mut rudof,
        &query_spec,
        &parsed_query_type,
        &parsed_result_format,
        &mut output_buffer,
    )
    .map_err(|e| {
        internal_error(
            "Query execution error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"execute_query"})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"utf8_conversion"})),
        )
    })?;
    // Calculate metadata
    let result_size_bytes = output_str.len();
    let result_lines = output_str.lines().count();

    let response = QueryExecutionResponse {
        query_type: query_type_str.clone(),
        result_format: result_format_str.to_string(),
        status: "success".to_string(),
        results: output_str.to_string(),
        result_size_bytes,
        result_lines,
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"serialize_response"})),
        )
    })?;

    // Create a summary text
    let summary = format!(
        "# SPARQL Query Execution\n\n\
        **Status:** ✓ Success\n\
        **Query Type:** {}\n\
        **Result Format:** {}\n\
        **Result Size:** {} bytes\n\
        **Result Lines:** {}\n",
        query_type_str, result_format_str, result_size_bytes, result_lines
    );

    let query_display = format!("## Query\n\n```sparql\n{}\n```", sparql_query);

    // Format results based on the format type
    let results_display = match result_format_str.to_lowercase().as_str() {
        "csv" => format!("## Results\n\n```csv\n{}\n```", output_str),
        "jsonld" | "json" => format!("## Results\n\n```json\n{}\n```", output_str),
        "turtle" | "n3" => format!("## Results\n\n```turtle\n{}\n```", output_str),
        "ntriples" | "nquads" => format!("## Results\n\n```ntriples\n{}\n```", output_str),
        "rdfxml" => format!("## Results\n\n```xml\n{}\n```", output_str),
        "trig" => format!("## Results\n\n```trig\n{}\n```", output_str),
        _ => format!("## Results\n\n```\n{}\n```", output_str),
    };

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(query_display),
        Content::text(results_display),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
