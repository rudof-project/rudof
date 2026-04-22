use crate::service::{errors::*, mcp_service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, CreateMessageRequestParams, SamplingMessage},
};
use rudof_lib::formats::{InputSpec, ResultQueryFormat};
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
    /// The SPARQL query that was executed (either provided directly or generated from natural language)
    pub query: String,
    /// Query results as a string
    pub results: String,
    /// Format used for the results
    pub result_format: String,
    /// Size of the results in bytes
    pub result_size_bytes: usize,
}

/// Generate a SPARQL query from natural language using rmcp sampling.
///
/// Uses the MCP sampling capability to request LLM assistance in
/// converting natural language to SPARQL.
async fn generate_sparql_from_natural_language(natural_language: &str) -> Result<String, McpError> {
    let system_message = r#"You are a SPARQL query expert. Convert natural language questions into valid SPARQL queries.
                                    - Only output the SPARQL query, no explanations or markdown formatting
                                    - Use standard SPARQL syntax (SELECT, CONSTRUCT, ASK, or DESCRIBE)
                                    - Include appropriate prefixes if needed
                                    - Make the query efficient and correct
                                    - If you need to guess prefixes, use common ones like rdf:, rdfs:, xsd:, ex:, etc."#;

    let user_message = format!("Generate a SPARQL query for: {}", natural_language);

    // Get the current request context to access the peer for sampling
    let context = RudofMcpService::current_request_context().ok_or_else(|| {
        internal_error(
            "Sampling context error",
            "Request context not found",
            Some(json!({"operation":"generate_sparql_from_natural_language","phase":"get_context"})),
        )
    })?;

    // Create sampling request with system prompt and user message.
    let sampling_request = CreateMessageRequestParams::new(vec![SamplingMessage::user_text(user_message.clone())], 512)
        .with_system_prompt(system_message)
        .with_temperature(0.1); // Low temperature for more deterministic output

    // Send sampling request through rmcp
    let response = context.peer.create_message(sampling_request).await.map_err(|e| {
        internal_error(
            "Query generation error",
            e.to_string(),
            Some(json!({"operation":"generate_sparql_from_natural_language","phase":"create_message"})),
        )
    })?;

    // Extract the first text fragment from the sampling response.
    let generated_query = response
        .message
        .content
        .iter()
        .find_map(|content| content.as_text().map(|text| text.text.clone()))
        .ok_or_else(|| {
            internal_error(
                "Sampling response error",
                "Expected text response from LLM",
                Some(json!({"operation":"generate_sparql_from_natural_language","phase":"extract_response_text"})),
            )
        })?;

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
            generate_sparql_from_natural_language(&nl).await?
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

    // Parse result format - return Tool Execution Error for invalid format
    let mut result_format_parsed = None;
    if let Some(result_format) = result_format {
        match ResultQueryFormat::from_str(&result_format) {
            Ok(fmt) => result_format_parsed = Some(fmt),
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid result format '{}': {}", result_format, e),
                    format!("Supported formats: {}", SPARQL_RESULT_FORMATS),
                )
                .into_call_tool_result());
            },
        }
    }

    // Guard: DESCRIBE is not yet implemented in rudof_lib.
    let query_type = sparql_query_type(&sparql_query);
    if matches!(query_type, Some("DESCRIBE")) {
        return Ok(ToolExecutionError::with_hint(
            "DESCRIBE queries are not yet supported",
            "Use SELECT, CONSTRUCT, or ASK queries instead",
        )
        .into_call_tool_result());
    }

    // Guard: SELECT queries only support the 'internal' result format.
    if matches!(query_type, Some("SELECT")) {
        if let Some(fmt) = &result_format_parsed {
            if !matches!(fmt, ResultQueryFormat::Internal) {
                return Ok(ToolExecutionError::with_hint(
                    format!("Format '{}' is not yet supported for SELECT queries", fmt),
                    "SELECT queries only support 'internal' format. Omit result_format or use 'internal'.",
                )
                .into_call_tool_result());
            }
        }
    }

    let query_spec = InputSpec::Str(sparql_query.clone());

    let mut rudof = service.rudof.lock().await;

    rudof.load_query(&query_spec).execute().map_err(|e| {
        internal_error(
            "Query execution error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"execute_query"})),
        )
    })?;

    rudof.run_query().execute().map_err(|e| {
        internal_error(
            "Query execution error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"run_query"})),
        )
    })?;

    let mut output_buffer = Cursor::new(Vec::new());

    let mut serialization = rudof.serialize_query_results(&mut output_buffer);
    if let Some(result_format_parsed) = &result_format_parsed {
        serialization = serialization.with_result_query_format(result_format_parsed);
    }
    serialization.execute().map_err(|e| {
        internal_error(
            "Query results serialization error",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"serialize_results"})),
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

    let result_format_str = if let Some(fmt) = result_format_parsed {
        fmt.to_string()
    } else {
        "internal".to_string()
    };
    let response = QueryExecutionResponse {
        query: sparql_query.clone(),
        result_format: result_format_str.clone(),
        results: output_str.clone(),
        result_size_bytes,
    };

    let structured = serialize_structured(&response, "execute_sparql_query_impl")?;

    let summary = format!(
        "SPARQL query executed.\nResult format: {}\nResult size: {} bytes",
        result_format_str, result_size_bytes
    );

    let query_display = code_block_preview("sparql", &sparql_query, 600);

    let results_language = match result_format_str.to_lowercase().as_str() {
        "csv" => "csv",
        "jsonld" | "json" => "json",
        "turtle" | "n3" => "turtle",
        "ntriples" | "nquads" => "ntriples",
        "rdfxml" => "xml",
        "trig" => "trig",
        _ => "text",
    };
    let results_display = code_block_preview(results_language, &output_str, DEFAULT_CONTENT_PREVIEW_CHARS);

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(format!("## Query Preview\n\n{}", query_display)),
        Content::text(format!("## Results Preview\n\n{}", results_display)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
