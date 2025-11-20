use crate::rudof_mcp_service::{errors::*, service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, CreateMessageRequestParam, Role, SamplingMessage},
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSparqlQueryRequest {
    /// SPARQL query string to execute (optional if query_natural_language is provided)
    pub query: Option<String>,

    /// Natural language description of the query to generate SPARQL using LLM
    pub query_natural_language: Option<String>,

    /// Result format
    pub result_format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryExecutionResponse {
    /// Query type that was executed
    pub query_type: String,

    /// Result format used
    pub result_format: String,

    /// Execution status
    pub status: String,

    /// Results as structured data
    pub results: String,

    /// Size of the results in bytes
    pub result_size_bytes: usize,

    /// Number of lines in the result
    pub result_lines: usize,
}

/// Generate a SPARQL query from natural language using rmcp sampling
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
            Some(
                json!({"operation":"generate_sparql_from_natural_language","phase":"get_context"}),
            ),
        )
    })?;

    // Create sampling request with messages
    let sampling_request = CreateMessageRequestParam {
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
    let response = context
        .peer
        .create_message(sampling_request)
        .await
        .map_err(|e| {
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
            Some(
                json!({"operation":"generate_sparql_from_natural_language","phase":"extract_response_text"}),
            ),
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

    tracing::info!(
        natural_language = %natural_language,
        generated_query = %cleaned_query,
        "Generated SPARQL query from natural language via rmcp sampling"
    );

    Ok(cleaned_query)
}

pub async fn execute_sparql_query_impl(
    service: &RudofMcpService,
    Parameters(ExecuteSparqlQueryRequest {
        query,
        query_natural_language,
        result_format,
    }): Parameters<ExecuteSparqlQueryRequest>,
) -> Result<CallToolResult, McpError> {
    // Determine the SPARQL query to execute
    let sparql_query = match (query, query_natural_language) {
        (Some(q), None) => {
            // Direct SPARQL query provided
            q
        }
        (None, Some(nl)) => {
            // Natural language query provided - generate SPARQL using rmcp sampling
            generate_sparql_from_natural_language(service, &nl).await?
        }
        (Some(_), Some(_)) => {
            return Err(invalid_request_error(
                "Invalid request",
                "Cannot provide both 'query' and 'query_natural_language'",
                Some(json!({"operation":"execute_sparql_query_impl", "phase":"query_validation"})),
            ));
        }
        (None, None) => {
            return Err(invalid_request_error(
                "Invalid request",
                "Must provide either 'query' or 'query_natural_language'",
                Some(json!({"operation":"execute_sparql_query_impl", "phase":"query_validation"})),
            ));
        }
    };

    let query_type_str = detect_query_type(&sparql_query).ok_or_else(|| {
        invalid_request_error(
            "Invalid query type",
            "Detecting query type error",
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"detect_query_type"})),
        )
    })?;
    let parsed_query_type = QueryType::from_str(&query_type_str).map_err(|e| {
        invalid_request_error(
            "Invalid query type",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"parse_query_type"})),
        )
    })?;

    let result_format_str = result_format.as_deref().unwrap_or("Internal");
    let parsed_result_format = ResultQueryFormat::from_str(result_format_str).map_err(|e| {
        invalid_request_error(
            "Invalid query result format",
            e.to_string(),
            Some(json!({"operation":"execute_sparql_query_impl", "phase":"parse_result_format"})),
        )
    })?;
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

    tracing::info!(
        query_type = %query_type_str,
        result_format = %result_format_str,
        result_size_bytes,
        result_lines,
        query_length = sparql_query.len(),
        "Executed SPARQL query successfully"
    );

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rudof_mcp_service::service::{RudofMcpService, ServiceConfig};
    use crate::rudof_mcp_service::tools::data_tools_impl::{
        LoadRdfDataFromSourcesRequest, load_rdf_data_from_sources_impl,
    };
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};

    const SAMPLE_TURTLE: &str = r#"
        prefix : <http://example.org/>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :a :name "Alice" ;
           :birthdate "1990-05-02"^^xsd:date ;
           :enrolledIn :cs101 .

        :b :name "Bob", "Robert" .

        :cs101 :name "Computer Science" .
    "#;

    async fn create_test_service() -> RudofMcpService {
        tokio::task::spawn_blocking(|| {
            let rudof_config = rudof_lib::RudofConfig::new().unwrap();
            let rudof = rudof_lib::Rudof::new(&rudof_config).unwrap();
            RudofMcpService {
                rudof: Arc::new(Mutex::new(rudof)),
                tool_router: Default::default(),
                prompt_router: Default::default(),
                config: Arc::new(RwLock::new(ServiceConfig::default())),
                resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
                current_min_log_level: Arc::new(RwLock::new(None)),
                current_context: Arc::new(RwLock::new(None)),
            }
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_execute_sparql_query_impl_select_success() {
        let service = create_test_service().await;

        let _ = load_rdf_data_from_sources_impl(
            &service,
            Parameters(LoadRdfDataFromSourcesRequest {
                data: vec![SAMPLE_TURTLE.to_string()],
                data_format: None,
                base: None,
                endpoint: None,
            }),
        )
        .await
        .unwrap();

        let query = r#"SELECT ?s ?p ?o WHERE { ?s ?p ?o }"#.to_string();

        let params = Parameters(ExecuteSparqlQueryRequest {
            query: Some(query),
            query_natural_language: None,
            result_format: Some("Internal".to_string()),
        });

        let result = execute_sparql_query_impl(&service, params).await;
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.structured_content.is_some());
        assert!(!call_result.content.is_empty());

        let structured = call_result.structured_content.unwrap();
        assert_eq!(structured["status"], "success");

        assert_eq!(
            structured["query_type"].as_str().unwrap().to_uppercase(),
            "SELECT"
        );
    }

    #[tokio::test]
    async fn test_execute_sparql_query_impl_invalid_query() {
        let service = create_test_service().await;

        let query = "INVALID QUERY".to_string();

        let params = Parameters(ExecuteSparqlQueryRequest {
            query: Some(query),
            query_natural_language: None,
            result_format: Some("Internal".to_string()),
        });

        let result = execute_sparql_query_impl(&service, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "Query execution error");
    }

    #[tokio::test]
    async fn test_execute_sparql_query_impl_invalid_result_format() {
        let service = create_test_service().await;

        let query = r#"SELECT ?s WHERE { ?s ?p ?o }"#.to_string();

        let params = Parameters(ExecuteSparqlQueryRequest {
            query: Some(query),
            query_natural_language: None,
            result_format: Some("UnknownFormat".to_string()),
        });

        let result = execute_sparql_query_impl(&service, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "Query execution error");
    }
}
